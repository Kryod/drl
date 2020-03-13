using UnityEngine;
using UnityEngine.UI;

namespace Algorithms
{
    public class DynaQ : AlgorithmConfigurator
    {
        [SerializeField] private Slider gamma;
        [SerializeField] private Slider nbIter;
        [SerializeField] private Slider n;
        [SerializeField] private Slider epsilon;
        [SerializeField] private Slider alpha;

        public override void Run()
        {
            NativePlugins.Drl.Instance.SetGamma(this.gamma.value);
            NativePlugins.Drl.Instance.SetNbIter((int)this.nbIter.value);
            NativePlugins.Drl.Instance.SetN((int)this.n.value);
            NativePlugins.Drl.Instance.SetEpsilon(this.epsilon.value);
            NativePlugins.Drl.Instance.SetAlpha(this.alpha.value);
            this.RunAlgorithm();
            this.DisplayQ();
        }
    }
}
