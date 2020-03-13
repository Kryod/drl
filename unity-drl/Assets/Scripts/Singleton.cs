using UnityEngine;

public class Singleton<T> : MonoBehaviour
where T : Singleton<T>
{
    public static T Instance { get; protected set; }

    public void Awake()
    {
        if (Instance != null && Instance != this)
        {
            Destroy(this);
            throw new System.Exception("An instance of " + this.GetType().Name + " already exists.");
        }
        else
        {
            Instance = (T)this;
        }
    }
}
