using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace UI
{
    public class AlgorithmSettings : MonoBehaviour
    {
        [SerializeField] private TMP_Dropdown algorithmTypeDropdown;
        [SerializeField] private RectTransform algorithmContainer;
        [SerializeField] private RectTransform[] algorithmSettings;
        [SerializeField] private Button runButton;

        private RectTransform spawnedSettings = null;

        public void Start()
        {
            this.algorithmTypeDropdown.onValueChanged.AddListener(this.OnAlgorithmTypeChanged);
            this.OnAlgorithmTypeChanged(0);
            this.runButton.onClick.AddListener(this.OnRunClicked);
        }

        private void OnAlgorithmTypeChanged(int idx)
        {
            if (spawnedSettings != null)
            {
                this.algorithmContainer.sizeDelta -= this.spawnedSettings.sizeDelta.y * Vector2.up;
                GameObject.Destroy(spawnedSettings.gameObject);
            }

            RectTransform prefab = this.algorithmSettings[idx];
            RectTransform instance = GameObject.Instantiate(prefab, this.algorithmContainer.transform);
            this.algorithmContainer.sizeDelta += instance.sizeDelta.y * Vector2.up;
            this.spawnedSettings = instance;
        }

        private void OnRunClicked()
        {
            AlgorithmConfigurator configurator = this.spawnedSettings.GetComponent<AlgorithmConfigurator>();
            if (configurator != null)
            {
                configurator.Run();
            }
        }
    }
}
