<template>
  <div class="toggle-container" :class="containerClass">
    <label v-if="label" class="toggle-label">{{ label }}</label>
    <div class="toggle-wrapper">
      <input
        v-model="localValue"
        type="checkbox"
        :disabled="disabled"
        class="toggle-input"
        @change="onChange"
      >
      <div class="toggle-track" :class="trackClass">
        <div class="toggle-thumb" :class="thumbClass" />
      </div>
      <span v-if="showLabel" class="toggle-text">
        {{ localValue ? onLabel : offLabel }}
      </span>
    </div>
    <div v-if="description" class="toggle-description">
      {{ description }}
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  modelValue: {
    type: Boolean,
    required: true
  },
  label: {
    type: String,
    default: ''
  },
  description: {
    type: String,
    default: ''
  },
  onLabel: {
    type: String,
    default: '开启'
  },
  offLabel: {
    type: String,
    default: '关闭'
  },
  showLabel: {
    type: Boolean,
    default: false
  },
  disabled: {
    type: Boolean,
    default: false
  },
  size: {
    type: String,
    default: 'medium',
    validator: (value) => ['small', 'medium', 'large'].includes(value)
  },
  variant: {
    type: String,
    default: 'primary',
    validator: (value) => ['primary', 'success', 'warning', 'danger'].includes(value)
  }
});

const emit = defineEmits(['update:modelValue', 'change']);

const localValue = ref(props.modelValue);

const containerClass = computed(() => [
  `toggle-container--${props.size}`,
  { 'toggle-container--disabled': props.disabled }
]);

const trackClass = computed(() => [
  `toggle-track--${props.size}`,
  `toggle-track--${props.variant}`,
  {
    'toggle-track--on': localValue.value,
    'toggle-track--disabled': props.disabled
  }
]);

const thumbClass = computed(() => [
  `toggle-thumb--${props.size}`,
  {
    'toggle-thumb--on': localValue.value,
    'toggle-thumb--disabled': props.disabled
  }
]);

const onChange = () => {
  emit('update:modelValue', localValue.value);
  emit('change', localValue.value);
};

watch(() => props.modelValue, (newValue) => {
  localValue.value = newValue;
});
</script>

<style scoped>
.toggle-container {
  width: 100%;
}

.toggle-label {
  display: block;
  font-size: 13px;
  color: var(--text-primary);
  margin-bottom: var(--spacing-xs);
  font-weight: 500;
}

.toggle-wrapper {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.toggle-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-track {
  position: relative;
  background: var(--border-color);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.toggle-track:hover:not(.toggle-track--disabled) {
  border-color: var(--border-color-light);
}

.toggle-track--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 尺寸变体 */
.toggle-track--small {
  width: 32px;
  height: 18px;
}

.toggle-track--medium {
  width: 40px;
  height: 22px;
}

.toggle-track--large {
  width: 48px;
  height: 26px;
}

/* 开启状态颜色 */
.toggle-track--on.toggle-track--primary {
  background: var(--info-color);
  border-color: var(--info-color);
}

.toggle-track--on.toggle-track--success {
  background: var(--success-color);
  border-color: var(--success-color);
}

.toggle-track--on.toggle-track--warning {
  background: var(--warning-color);
  border-color: var(--warning-color);
}

.toggle-track--on.toggle-track--danger {
  background: var(--danger-color);
  border-color: var(--danger-color);
}

.toggle-thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  background: var(--text-primary);
  border-radius: 50%;
  transition: all var(--transition-fast);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

/* 滑块尺寸 */
.toggle-thumb--small {
  width: 14px;
  height: 14px;
}

.toggle-thumb--medium {
  width: 18px;
  height: 18px;
}

.toggle-thumb--large {
  width: 22px;
  height: 22px;
}

/* 开启状态位置 */
.toggle-thumb--on.toggle-thumb--small {
  transform: translateX(14px);
}

.toggle-thumb--on.toggle-thumb--medium {
  transform: translateX(18px);
}

.toggle-thumb--on.toggle-thumb--large {
  transform: translateX(22px);
}

.toggle-text {
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
  min-width: 24px;
}

.toggle-description {
  margin-top: var(--spacing-xs);
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.4;
}

.toggle-container--disabled .toggle-label,
.toggle-container--disabled .toggle-text {
  opacity: 0.5;
}
</style> 