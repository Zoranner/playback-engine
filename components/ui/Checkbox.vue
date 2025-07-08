<template>
  <div class="checkbox-container" :class="containerClass">
    <label class="checkbox-wrapper">
      <input
        v-model="localValue"
        type="checkbox"
        :disabled="disabled"
        :indeterminate="indeterminate"
        class="checkbox-input"
        @change="onChange"
      >
      <div class="checkbox-indicator" :class="indicatorClass">
            <Icon v-if="isChecked" name="heroicons:check" :size="iconSize" class="checkbox-icon" />
    <Icon v-else-if="indeterminate" name="heroicons:minus" :size="iconSize" class="checkbox-icon" />
      </div>
      <span v-if="label || $slots.default" class="checkbox-label">
        <slot>{{ label }}</slot>
      </span>
    </label>
    <div v-if="description" class="checkbox-description">
      {{ description }}
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';


const props = defineProps({
  modelValue: {
    type: [Boolean, Array],
    required: true
  },
  value: {
    type: [String, Number, Boolean],
    default: null
  },
  label: {
    type: String,
    default: ''
  },
  description: {
    type: String,
    default: ''
  },
  disabled: {
    type: Boolean,
    default: false
  },
  indeterminate: {
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
  `checkbox-container--${props.size}`,
  { 'checkbox-container--disabled': props.disabled }
]);

const indicatorClass = computed(() => [
  `checkbox-indicator--${props.size}`,
  `checkbox-indicator--${props.variant}`,
  {
    'checkbox-indicator--checked': isChecked.value,
    'checkbox-indicator--indeterminate': props.indeterminate,
    'checkbox-indicator--disabled': props.disabled
  }
]);

const isChecked = computed(() => {
  if (Array.isArray(localValue.value)) {
    return localValue.value.includes(props.value);
  }
  return localValue.value;
});

const iconSize = computed(() => {
  const sizeMap = {
    small: 10,
    medium: 12,
    large: 14
  };
  return sizeMap[props.size];
});

const onChange = (event) => {
  if (Array.isArray(localValue.value)) {
    const newValue = [...localValue.value];
    if (event.target.checked) {
      newValue.push(props.value);
    } else {
      const index = newValue.indexOf(props.value);
      if (index > -1) {
        newValue.splice(index, 1);
      }
    }
    localValue.value = newValue;
  } else {
    localValue.value = event.target.checked;
  }
  
  emit('update:modelValue', localValue.value);
  emit('change', localValue.value);
};

watch(() => props.modelValue, (newValue) => {
  localValue.value = newValue;
});
</script>

<style scoped>
.checkbox-container {
  width: 100%;
}

.checkbox-wrapper {
  display: flex;
  align-items: flex-start;
  gap: var(--spacing-sm);
  cursor: pointer;
  user-select: none;
}

.checkbox-input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.checkbox-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xs);
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.checkbox-indicator:hover:not(.checkbox-indicator--disabled) {
  border-color: var(--border-color-light);
  background: var(--tertiary-bg);
}

/* 尺寸变体 */
.checkbox-indicator--small {
  width: 14px;
  height: 14px;
}

.checkbox-indicator--medium {
  width: 16px;
  height: 16px;
}

.checkbox-indicator--large {
  width: 18px;
  height: 18px;
}

/* 选中状态颜色 */
.checkbox-indicator--checked.checkbox-indicator--primary,
.checkbox-indicator--indeterminate.checkbox-indicator--primary {
  background: var(--info-color);
  border-color: var(--info-color);
}

.checkbox-indicator--checked.checkbox-indicator--success,
.checkbox-indicator--indeterminate.checkbox-indicator--success {
  background: var(--success-color);
  border-color: var(--success-color);
}

.checkbox-indicator--checked.checkbox-indicator--warning,
.checkbox-indicator--indeterminate.checkbox-indicator--warning {
  background: var(--warning-color);
  border-color: var(--warning-color);
}

.checkbox-indicator--checked.checkbox-indicator--danger,
.checkbox-indicator--indeterminate.checkbox-indicator--danger {
  background: var(--danger-color);
  border-color: var(--danger-color);
}

.checkbox-indicator--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.checkbox-icon {
  color: var(--primary-bg);
}

.checkbox-label {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
  line-height: 1.4;
  margin-top: 1px;
}

.checkbox-description {
  margin-top: var(--spacing-xs);
  margin-left: calc(var(--spacing-sm) + 16px);
  font-size: 11px;
  color: var(--text-muted);
  line-height: 1.4;
}

.checkbox-container--disabled .checkbox-wrapper {
  cursor: not-allowed;
}

.checkbox-container--disabled .checkbox-label {
  opacity: 0.5;
}
</style> 