using System;

#if UNITY_EDITOR
using System.Runtime.InteropServices;
using UnityEngine;

namespace Utils
{
    // This class allows to dynamically load and unload a native plugin and to call its functions.
    // 
    // Such a loader is really useful in the Unity Editor since it doesn't unload native plugins after exitting play mode,
    // making it impossible to copy a newer version of the library when you rebuild it after changing its source code.
    public static class NativePluginLoader
    {
#if !UNITY_EDITOR_WIN
#if UNITY_EDITOR_OSX
        private const string LibDl = "libdl.dylib";
#else // Linux
        private const string LibDl = "libdl.so";
#endif
#endif
        public static T GetDelegate<T>(IntPtr handle, string name)
        {
            if (string.IsNullOrEmpty(name))
            {
                name = typeof(T).Name;
            }
            var funcPtr = GetProcAddress(handle, name);
            if (funcPtr == IntPtr.Zero)
            {
                Debug.LogError("Could not get reference to method address for " + name + ".");
                return default;
            }

            var deleg = Marshal.GetDelegateForFunctionPointer(funcPtr, typeof(T));
            return (T)Convert.ChangeType(deleg, typeof(T));
        }

#if UNITY_EDITOR_WIN
        [DllImport("kernel32", EntryPoint = "LoadLibrary", SetLastError = true, CharSet = CharSet.Unicode)]
        public static extern IntPtr LoadPlugin(string lpFileName);

        [DllImport("kernel32", EntryPoint = "FreeLibrary", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool FreePlugin(IntPtr hModule);

        [DllImport("kernel32")]
        private static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);
#else // macOS & Linux
        [DllImport(NativePluginLoader.LibDl)]
        private static extern IntPtr dlopen(String fileName, int flags);

        [DllImport(NativePluginLoader.LibDl)]
        private static extern IntPtr dlsym(IntPtr handle, String symbol);

        [DllImport(NativePluginLoader.LibDl, EntryPoint = "dlclose")]
        public static extern int FreePlugin(IntPtr handle);

        [DllImport(NativePluginLoader.LibDl)]
        private static extern IntPtr dlerror();

        const int RTLD_NOW = 0x00002;

        public static IntPtr LoadPlugin(string fileName) {
#if UNITY_EDITOR_OSX
            fileName += ".dylib";
#else
            fileName += ".so";
#endif
            var res = dlopen(fileName, RTLD_NOW);
            var errPtr = dlerror();
            if (errPtr != IntPtr.Zero) {
                throw new Exception("dlopen: " + Marshal.PtrToStringAnsi(errPtr));
            }
            return res;
        }

        public static IntPtr GetProcAddress(IntPtr dllHandle, string name) {
            // Clear previous errors if any
            dlerror();
            var res = dlsym(dllHandle, name);
            var errPtr = dlerror();
            if (errPtr != IntPtr.Zero) {
                throw new Exception("dlsym: " + Marshal.PtrToStringAnsi(errPtr));
            }
            return res;
        }
#endif // #if UNITY_EDITOR_WIN
    }
}
#endif // #if UNITY_EDITOR

namespace Utils
{
    public abstract class NativePlugin
    {
        protected IntPtr DllHandle;
        protected IntPtr Handle;

        public bool IsHandleValid()
        {
            var valid = this.Handle != IntPtr.Zero;
#if UNITY_EDITOR
            valid = valid && this.DllHandle != IntPtr.Zero;
#endif
            return valid;
        }

        protected void CheckHandle()
        {
            var err = "This " + this.GetType().Name + " instance cannot be used because it has been disposed or has not been loaded properly.";
#if UNITY_EDITOR
            if (this.DllHandle == IntPtr.Zero)
            {
                throw new AccessViolationException(err);
            }
#endif
            if (this.Handle == IntPtr.Zero)
            {
                throw new AccessViolationException(err);
            }
        }
    }
}
