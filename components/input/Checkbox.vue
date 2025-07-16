<template>
  <div :class="containerClasses">
    <label class="flex cursor-pointer select-none items-start gap-sm">
      <input
        v-model="localValue"
        type="checkbox"
        :disabled="disabled"
        :indeterminate="indeterminate"
        class="absolute h-0 w-0 opacity-0"
        @change="onChange"
      />

      <div :class="indicatorClasses">
        <Icon
          v-if="isChecked"
          name="heroicons:check"
          class="h-3 w-3 text-background-primary"
        />
        <Icon
          v-else-if="indeterminate"
          name="heroicons:minus"
          class="h-3 w-3 text-background-primary"
        />
      </div>

      <span
        v-if="label || $slots.default"
        class="mt-0.5 flex-1 leading-normal text-text-primary"
      >
        <slot>{{ label }}</slot>
      </span>
    </label>

    <div
      v-if="description"
      class="ml-7 mt-xs leading-normal text-text-muted"
    >
      {{ description }}
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  modelValue: {
    type: [Boolean, Array],
    required: true,
  },
  value: {
    type: [String, Number, Boolean],
    default: null,
  },
  label: {
    type: String,
    default: '',
  },
  description: {
    type: String,
    default: '',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  indeterminate: {
    type: Boolean,
    default: false,
  },
  variant: {
    type: String,
    default: 'primary',
    validator: value => ['primary', 'success', 'warning', 'danger'].includes(value),
  },
});

const emit = defineEmits(['update:modelValue', 'change']);

const localValue = ref(props.modelValue);

// 容器样式类
const containerClasses = computed(() => ['w-full', props.disabled && 'opacity-50']);

// 指示器样式类
const indicatorClasses = computed(() => {
  const baseClasses = [
    'flex items-center justify-center w-4 h-4 rounded-xs border transition-all duration-fast flex-shrink-0',
  ];

  if (props.disabled) {
    baseClasses.push('opacity-50 cursor-not-allowed');
  } else {
    baseClasses.push('hover:border-border-light hover:bg-background-tertiary');
  }

  // 选中或不确定状态的颜色
  if (isChecked.value || props.indeterminate) {
    const variantClasses = {
      primary: ['bg-info border-info'],
      success: ['bg-success border-success'],
      warning: ['bg-warning border-warning'],
      danger: ['bg-danger border-danger'],
    };
    baseClasses.push(...(variantClasses[props.variant] || variantClasses.primary));
  } else {
    baseClasses.push('bg-background-secondary border-border');
  }

  return baseClasses;
});

const isChecked = computed(() => {
  if (Array.isArray(localValue.value)) {
    return localValue.value.includes(props.value);
  }
  return localValue.value;
});

const onChange = event => {
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

watch(
  () => props.modelValue,
  newValue => {
    localValue.value = newValue;
  }
);
</script>
