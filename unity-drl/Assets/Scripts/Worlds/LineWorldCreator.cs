using System;
using UnityEngine;
using UnityEngine.UI;

namespace Worlds
{
    public class LineWorldCreator : WorldCreator
    {
        [SerializeField] private Slider cellsSlider;

        public override void CreateWorld()
        {
            UI.GridManager.Instance.Reset();
            UI.GridManager.Instance.CreateLine((int)this.cellsSlider.value);

            NativePlugins.Drl.Instance.InitLineWorld((UIntPtr)this.cellsSlider.value);
        }
    }
}
