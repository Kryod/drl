using UnityEngine;
using UnityEngine.UI;

namespace Algorithms
{
    public class Sarsa : AlgorithmConfigurator
    {
        [SerializeField] private Slider gamma;
        [SerializeField] private Slider nbIter;
        [SerializeField] private Slider maxStep;
        [SerializeField] private Slider epsilon;
        [SerializeField] private Slider alpha;

        public override void Run()
        {
            NativePlugins.Drl.Instance.SetGamma(this.gamma.value);
            NativePlugins.Drl.Instance.SetNbIter((int)this.nbIter.value);
            NativePlugins.Drl.Instance.SetMaxStep((int)this.maxStep.value);
            NativePlugins.Drl.Instance.SetEpsilon(this.epsilon.value);
            NativePlugins.Drl.Instance.SetAlpha(this.alpha.value);
            this.RunAlgorithm();
            this.DisplayQ();
        }
    }
}
