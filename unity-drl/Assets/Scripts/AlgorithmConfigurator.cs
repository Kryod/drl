using System;
using UnityEngine;
using NativePlugins;
using NativePlugins.Types;

public abstract class AlgorithmConfigurator : MonoBehaviour
{
    [SerializeField] protected Algorithm algorithm;

    public virtual void Run()
    {
        this.RunAlgorithm();
    }

    protected virtual unsafe void RunAlgorithm()
    {
        Drl.Instance.SetNbIter(1000);
        Drl.Instance.Run(this.algorithm);
    }

    protected unsafe void DisplayV()
    {
        UIntPtr size;
        float* V = Drl.Instance.GetV(&size);
        for (int i = 0; i < (int)size; ++i)
        {
            float v = *(V + i);
            InitCell(i, v, 0.0f);
        }
    }

    protected unsafe void DisplayQ()
    {
        UIntPtr sizeUsize, nbActionsUsize;
        float* Q = Drl.Instance.GetQ(&sizeUsize, &nbActionsUsize);
        int size = (int)sizeUsize;
        int nbActions = (int)nbActionsUsize;

        int cols = UI.GridManager.Instance.cols;
        for (int i = 0; i < size; i += nbActions)
        {
            int maxIdx = -1;
            float max = 0.0f;
            for (int j = 0; j < nbActions; ++j)
            {
                float val = *(Q + i + j);
                if (maxIdx == -1 || val > max)
                {
                    max = val;
                    maxIdx = j;
                }
            }
            InitCell(i / nbActions, max, MakeRotation(nbActions, maxIdx));
        }
    }

    protected unsafe void DisplayPi()
    {
        UIntPtr sizeUsize, nbActionsUsize;
        float* Pi = Drl.Instance.GetPi(&sizeUsize, &nbActionsUsize);
        int size = (int)sizeUsize;
        int nbActions = (int)nbActionsUsize;

        int cols = UI.GridManager.Instance.cols;
        for (int i = 0; i < size; i += nbActions)
        {
            int maxIdx = -1;
            float max = 0.0f;
            for (int j = 0; j < nbActions; ++j)
            {
                float val = *(Pi + i + j);
                if (maxIdx == -1 || val > max)
                {
                    max = val;
                    maxIdx = j;
                }
            }
            UI.GridCell cell = UI.GridManager.Instance.Get(i / nbActions);
            InitArrow(cell, MakeRotation(nbActions, maxIdx));
        }
    }

    private static void InitCell(int idx, float value, float rotation)
    {
        UI.GridCell cell = UI.GridManager.Instance.Get(idx);
        cell.text.text = FormatNumberTruncate(value, 5);
        cell.text.fontSize = cell.text.rectTransform.rect.width * 0.2f;
        InitArrow(cell, rotation);
    }

    private static void InitArrow(UI.GridCell cell, float rotation)
    {
        Color color = cell.arrow.color;
        color.a = 0.7f;
        cell.arrow.color = color;
        cell.arrow.transform.rotation = Quaternion.AngleAxis(rotation, Vector3.forward);
        cell.arrowContainer.sizeDelta = new Vector2(cell.image.rectTransform.rect.width * 0.3f, cell.image.rectTransform.rect.height * 0.3f);
    }

    private static float MakeRotation(int nbActions, int idx)
    {
        if (nbActions == 2)
        {
            return idx == 0 ? 180.0f : 0.0f;
        }
        else if (nbActions == 4)
        {
            if (idx == 0) return 180.0f;
            else if (idx == 1) return 0.0f;
            else if (idx == 2) return 90.0f;
            else if (idx == 3) return -90.0f;
        }
        return 0.0f;
    }

    private static string FormatNumberTruncate(float d, int decimalPlaces)
    {
        float factor = Mathf.Pow(10, decimalPlaces);
        float truncated = Mathf.Floor(d * factor) / factor;
        return truncated.ToString();
    }
}
