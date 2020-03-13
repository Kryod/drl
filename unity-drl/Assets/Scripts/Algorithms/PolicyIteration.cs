using UnityEngine;
using UnityEngine.UI;

namespace Algorithms
{
    public class PolicyIteration : AlgorithmConfigurator
    {
        [SerializeField] private Slider theta;
        [SerializeField] private Slider gamma;

        public override void Run()
        {
            NativePlugins.Drl.Instance.SetTheta(this.theta.value);
            NativePlugins.Drl.Instance.SetGamma(this.gamma.value);
            this.RunAlgorithm();
            this.DisplayV();
            this.DisplayPi();
        }
    }
}
