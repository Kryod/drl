using System;
using UnityEngine;
using NativePlugins;
using NativePlugins.Types;

public abstract class AlgorithmConfigurator : MonoBehaviour
{
    [SerializeField] protected Algorithm algorithm;

    public void Run()
    {
        this.RunAlgorithm();
    }

    protected unsafe void RunAlgorithm()
    {
        Drl drl = Drl.Instance;
        drl.Run(this.algorithm);

        UIntPtr size;
        float* V = drl.GetV(&size);
        for (int i = 0; i < (int)size; ++i)
        {
            float v = *(V + i);
            UI.GridCell cell = UI.GridManager.Instance.Get(i);
            cell.text.text = v.ToString("0.0000");
        }
    }
}
