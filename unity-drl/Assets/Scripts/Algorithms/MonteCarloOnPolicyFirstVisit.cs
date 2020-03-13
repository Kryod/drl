using UnityEngine;
using UnityEngine.UI;

namespace Algorithms
{
    public class MonteCarloOnPolicyFirstVisit : AlgorithmConfigurator
    {
        [SerializeField] private Slider gamma;
        [SerializeField] private Slider epsilon;
        [SerializeField] private Slider nbIter;

        public override void Run()
        {
            NativePlugins.Drl.Instance.SetGamma(this.gamma.value);
            NativePlugins.Drl.Instance.SetEpsilon(this.epsilon.value);
            NativePlugins.Drl.Instance.SetNbIter((int)this.nbIter.value);
            this.RunAlgorithm();
            this.DisplayQ();
        }
    }
}
