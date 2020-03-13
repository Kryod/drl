using System;
using UnityEngine;
using UnityEngine.UI;

namespace Worlds
{
    public class GridWorldCreator : WorldCreator
    {
        [SerializeField] private Slider widthSlider;
        [SerializeField] private Slider heightSlider;

        public override void CreateWorld()
        {
            UI.GridManager.Instance.Reset();
            UI.GridManager.Instance.CreateGrid((int)this.widthSlider.value, (int)this.heightSlider.value);

            NativePlugins.Drl.Instance.InitGridWorld(
                (UIntPtr)this.widthSlider.value,
                (UIntPtr)this.heightSlider.value
            );
        }
    }
}
