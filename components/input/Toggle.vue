<template>
  <div :class="containerClasses">
    <label v-if="label" class="mb-xs block font-medium text-text-primary">
      {{ label }}
    </label>

    <div class="flex items-center gap-sm">
      <input
        v-model="localValue"
        type="checkbox"
        :disabled="disabled"
        class="absolute h-0 w-0 opacity-0"
        @change="onChange"
      />

      <div :class="trackClasses" @click="!disabled && toggle()">
        <div :class="thumbClasses" />
      </div>

      <span v-if="showLabel" class="min-w-6 font-medium text-text-secondary">
        {{ localValue ? onLabel : offLabel }}
      </span>
    </div>

    <div v-if="description" class="mt-xs leading-normal text-text-muted">
      {{ description }}
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  modelValue: {
    type: Boolean,
    required: true,
  },
  label: {
    type: String,
    default: '',
  },
  description: {
    type: String,
    default: '',
  },
  onLabel: {
    type: String,
    default: '开启',
  },
  offLabel: {
    type: String,
    default: '关闭',
  },
  showLabel: {
    type: Boolean,
    default: false,
  },
  disabled: {
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

// 轨道样式类
const trackClasses = computed(() => {
  const baseClasses = [
    'relative w-10 h-5.5 rounded-full cursor-pointer transition-all duration-fast flex-shrink-0',
    'border',
  ];

  if (props.disabled) {
    baseClasses.push('opacity-50 cursor-not-allowed');
  } else {
    baseClasses.push('hover:border-border-light');
  }

  // 颜色变体
  if (localValue.value) {
    const variantClasses = {
      primary: ['bg-info border-info'],
      success: ['bg-success border-success'],
      warning: ['bg-warning border-warning'],
      danger: ['bg-danger border-danger'],
    };
    baseClasses.push(...(variantClasses[props.variant] || variantClasses.primary));
  } else {
    baseClasses.push('bg-border border-border');
  }

  return baseClasses;
});

// 滑块样式类
const thumbClasses = computed(() => [
  'absolute top-0.5 left-0.5 w-4.5 h-4.5 bg-text-primary rounded-full',
  'transition-transform duration-fast shadow-sm',
  localValue.value && 'translate-x-4.5',
]);

const toggle = () => {
  localValue.value = !localValue.value;
  onChange();
};

const onChange = () => {
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
