using TMPro;
using UnityEngine;
using UnityEngine.UI;

namespace UI
{
    public class WorldSettings : MonoBehaviour
    {
        [SerializeField] private TMP_Dropdown worldTypeDropdown;
        [SerializeField] private RectTransform worldContainer;
        [SerializeField] private RectTransform[] worldSettings;
        [SerializeField] private Button createButton;

        private RectTransform spawnedSettings = null;

        public void Start()
        {
            this.worldTypeDropdown.onValueChanged.AddListener(this.OnWorldTypeChanged);
            this.OnWorldTypeChanged(0);
            this.createButton.onClick.AddListener(this.OnCreateClicked);
        }

        private void OnWorldTypeChanged(int idx)
        {
            if (spawnedSettings != null)
            {
                this.worldContainer.sizeDelta -= this.spawnedSettings.sizeDelta.y * Vector2.up;
                GameObject.Destroy(spawnedSettings.gameObject);
            }

            RectTransform prefab = this.worldSettings[idx];
            RectTransform instance = GameObject.Instantiate(prefab, this.worldContainer.transform);
            this.worldContainer.sizeDelta += instance.sizeDelta.y * Vector2.up;
            this.spawnedSettings = instance;
        }

        private void OnCreateClicked()
        {
            WorldCreator creator = this.spawnedSettings.GetComponent<WorldCreator>();
            if (creator != null)
            {
                creator.CreateWorld();
            }
        }
    }
}
