using UnityEngine;

namespace NativePlugins
{
    public class DrlBehaviour : MonoBehaviour
    {
        private void Awake()
        {
            Drl.Initialize();
        }

        private void OnApplicationQuit()
        {
            if (Drl.Instance != null)
            {
                Drl.Instance.Dispose();
            }
        }
    }
}
