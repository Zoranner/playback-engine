<template>
  <div class="slider-container">
    <label v-if="label" class="slider-label">{{ label }}</label>
    <div class="slider-wrapper">
      <span v-if="showMinMax" class="slider-value text-monospace">{{ formatValue(min) }}</span>
      <div ref="trackRef" class="slider-track" @click="onTrackClick">
        <input
          v-model="localValue"
          type="range"
          :min="min"
          :max="max"
          :step="step"
          :disabled="disabled"
          class="slider-input"
          @input="onInput"
          @change="onChange"
        >
        <div class="slider-progress" :style="{ width: progressPercentage + '%' }"/>
        <div 
          class="slider-thumb" 
          :style="{ left: progressPercentage + '%' }"
          :class="{ 'slider-thumb--dragging': isDragging }"
        />
      </div>
      <span v-if="showMinMax" class="slider-value text-monospace">{{ formatValue(max) }}</span>
    </div>
    <div v-if="showValue" class="slider-current">
      <span class="text-monospace">{{ formatValue(localValue) }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  modelValue: {
    type: [Number, String],
    required: true
  },
  min: {
    type: Number,
    default: 0
  },
  max: {
    type: Number,
    default: 100
  },
  step: {
    type: Number,
    default: 1
  },
  label: {
    type: String,
    default: ''
  },
  disabled: {
    type: Boolean,
    default: false
  },
  showValue: {
    type: Boolean,
    default: false
  },
  showMinMax: {
    type: Boolean,
    default: false
  },
  formatter: {
    type: Function,
    default: (value) => value.toString()
  },
  variant: {
    type: String,
    default: 'primary',
    validator: (value) => ['primary', 'success', 'warning', 'danger'].includes(value)
  }
});

const emit = defineEmits(['update:modelValue', 'change', 'input']);

const trackRef = ref(null);
const isDragging = ref(false);
const localValue = ref(props.modelValue);

const progressPercentage = computed(() => {
  return ((localValue.value - props.min) / (props.max - props.min)) * 100;
});

const formatValue = (value) => {
  return props.formatter(value);
};

const onInput = (event) => {
  localValue.value = Number(event.target.value);
  emit('update:modelValue', localValue.value);
  emit('input', localValue.value);
};

const onChange = (event) => {
  emit('change', Number(event.target.value));
};

const onTrackClick = (event) => {
  if (props.disabled) return;
  
  const track = trackRef.value;
  const rect = track.getBoundingClientRect();
  const percentage = (event.clientX - rect.left) / rect.width;
  const newValue = props.min + (props.max - props.min) * percentage;
  
  localValue.value = Math.round(newValue / props.step) * props.step;
  emit('update:modelValue', localValue.value);
  emit('change', localValue.value);
};

watch(() => props.modelValue, (newValue) => {
  localValue.value = newValue;
});
</script>

<style scoped>
.slider-container {
  width: 100%;
}

.slider-label {
  display: block;
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
  font-weight: 500;
}

.slider-wrapper {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.slider-track {
  position: relative;
  flex: 1;
  height: 4px;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  min-width: 100px;
}

.slider-input {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  cursor: pointer;
  z-index: 3;
}

.slider-input:disabled {
  cursor: not-allowed;
}

.slider-progress {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: linear-gradient(90deg, var(--info-color), var(--text-accent));
  border-radius: var(--radius-sm);
  transition: width var(--transition-fast);
  box-shadow: 0 0 2px var(--info-color);
  z-index: 1;
}

.slider-thumb {
  position: absolute;
  top: 50%;
  width: 12px;
  height: 12px;
  background: var(--text-accent);
  border: 2px solid var(--primary-bg);
  border-radius: 50%;
  transform: translate(-50%, -50%);
  box-shadow: 0 0 4px rgba(56, 189, 248, 0.3);
  transition: all var(--transition-fast);
  z-index: 2;
  cursor: pointer;
}

.slider-thumb:hover,
.slider-thumb--dragging {
  transform: translate(-50%, -50%) scale(1.2);
  box-shadow: 0 0 8px rgba(56, 189, 248, 0.5);
}

.slider-value {
  font-size: 10px;
  color: var(--text-secondary);
  min-width: 32px;
  text-align: center;
}

.slider-current {
  margin-top: var(--spacing-xs);
  text-align: center;
}

.slider-current .text-monospace {
  font-size: 11px;
  color: var(--text-accent);
  font-weight: 600;
}

/* 变体样式 */
.slider-container--success .slider-progress {
  background: linear-gradient(90deg, var(--success-color), #34d399);
  box-shadow: 0 0 2px var(--success-color);
}

.slider-container--warning .slider-progress {
  background: linear-gradient(90deg, var(--warning-color), #fbbf24);
  box-shadow: 0 0 2px var(--warning-color);
}

.slider-container--danger .slider-progress {
  background: linear-gradient(90deg, var(--danger-color), #f87171);
  box-shadow: 0 0 2px var(--danger-color);
}
</style> 