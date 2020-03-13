using TMPro;
using System;
using System.Globalization;
using UnityEngine;
using UnityEngine.UI;

namespace UI
{
    public class SliderInputLink : MonoBehaviour
    {
        [SerializeField] private Slider slider;
        [SerializeField] private TMP_InputField input;

        public Action<float> OnValueChanged;

        private void Awake()
        {
            Action<string> onInputChange = s =>
            {
                if (!this.slider.wholeNumbers)
                {
                    var culture = CultureInfo.GetCultureInfo("en-US").NumberFormat;
                    if (!float.TryParse(s, System.Globalization.NumberStyles.Float, culture, out float val))
                    {
                        return;
                    }
                    this.SetValue(val);
                    this.OnValueChanged?.Invoke(val);
                }
                else
                {
                    if (!int.TryParse(s, out int val))
                    {
                        return;
                    }
                    this.SetValue(val);
                    this.OnValueChanged?.Invoke(val);
                }
            };
            this.input.onSubmit.AddListener(s => onInputChange(s));
            this.input.onValueChanged.AddListener(s => onInputChange(s));

            this.slider.onValueChanged.AddListener(val =>
            {
                this.OnValueChanged?.Invoke(val);
                this.input.text = val.ToString();
            });

            this.slider.onValueChanged.Invoke(this.slider.value);
        }

        public void SetValue(float val)
        {
            if (val > this.slider.maxValue)
            {
                this.slider.maxValue = val;
            }
            if (val < this.slider.minValue)
            {
                this.slider.minValue = val;
            }
            this.slider.value = val;
            this.input.text = val.ToString();
        }
    }
}
