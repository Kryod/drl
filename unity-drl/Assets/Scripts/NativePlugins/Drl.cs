namespace NativePlugins
{
    public class Drl : DrlBase
    {
        public static Drl Instance { get; private set; } = null;

        public static void Initialize()
        {
            if (Instance == null)
            {
                Instance = new Drl();
            }
        }
    }
}
