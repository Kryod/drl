using UnityEngine;
using UnityEngine.UI;

namespace Algorithms
{
    public class ValueIteration : AlgorithmConfigurator
    {
        [SerializeField] private Slider gamma;
        [SerializeField] private Slider theta;

        public override void Run()
        {
            NativePlugins.Drl.Instance.SetGamma(this.gamma.value);
            NativePlugins.Drl.Instance.SetTheta(this.theta.value);
            this.RunAlgorithm();
            this.DisplayV();
            this.DisplayPi();
        }
    }
}
