<template>
  <div class="w-full">
    <label
      v-if="label"
      class="mb-xs block font-medium text-text-secondary"
    >
      {{ label }}
    </label>

    <div class="flex items-center gap-sm">
      <span
        v-if="showMinMax"
        class="font-mono text-xs text-text-secondary"
      >
        {{ formatValue(min) }}
      </span>

      <div
        ref="trackRef"
        :class="trackClasses"
        @click="onTrackClick"
      >
        <input
          v-model="localValue"
          type="range"
          :min="min"
          :max="max"
          :step="step"
          :disabled="disabled"
          class="absolute left-0 top-0 z-30 h-full w-full cursor-pointer opacity-0"
          @input="onInput"
          @change="onChange"
        />

        <!-- 进度条 -->
        <div
          :class="progressClasses"
          :style="{ width: progressPercentage + '%' }"
        />

        <!-- 拖拽点 -->
        <div
          :class="thumbClasses"
          :style="{ left: progressPercentage + '%' }"
        />
      </div>

      <span
        v-if="showMinMax"
        class="font-mono text-xs text-text-secondary"
      >
        {{ formatValue(max) }}
      </span>
    </div>

    <div
      v-if="showValue"
      class="mt-xs text-center"
    >
      <span class="font-mono text-sm text-text-accent">
        {{ formatValue(localValue) }}
      </span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  modelValue: {
    type: [Number, String],
    required: true,
  },
  min: {
    type: Number,
    default: 0,
  },
  max: {
    type: Number,
    default: 100,
  },
  step: {
    type: Number,
    default: 1,
  },
  label: {
    type: String,
    default: '',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  showValue: {
    type: Boolean,
    default: false,
  },
  showMinMax: {
    type: Boolean,
    default: false,
  },
  formatter: {
    type: Function,
    default: value => value.toString(),
  },
  variant: {
    type: String,
    default: 'primary',
    validator: value => ['primary', 'success', 'warning', 'danger'].includes(value),
  },
});

const emit = defineEmits(['update:modelValue', 'change', 'input']);

const trackRef = ref(null);
const localValue = ref(props.modelValue);

const progressPercentage = computed(() => {
  return ((localValue.value - props.min) / (props.max - props.min)) * 100;
});

// 轨道样式类
const trackClasses = computed(() => [
  'relative flex-1 h-2 bg-background-secondary border border-border rounded-sm cursor-pointer min-w-25',
  props.disabled && 'opacity-50 cursor-not-allowed',
]);

// 进度条样式类
const progressClasses = computed(() => {
  const baseClasses = [
    'absolute top-0 left-0 h-2 rounded-sm transition-[width] duration-fast z-10',
  ];

  const variantClasses = {
    primary: ['bg-gradient-to-r from-info to-text-accent', 'shadow-sm shadow-info/30'],
    success: ['bg-gradient-to-r from-success to-success', 'shadow-sm shadow-success/30'],
    warning: ['bg-gradient-to-r from-warning to-warning', 'shadow-sm shadow-warning/30'],
    danger: ['bg-gradient-to-r from-danger to-danger', 'shadow-sm shadow-danger/30'],
  };

  return [...baseClasses, ...(variantClasses[props.variant] ?? variantClasses.primary)];
});

// 拖拽点样式类
const thumbClasses = computed(() => [
  'absolute top-1/2 w-3 h-3 bg-text-accent border-2 border-background-primary',
  'rounded-full transform -translate-x-1/2 -translate-y-1/2 cursor-pointer',
  'transition-all duration-fast z-20 shadow-sm shadow-primary/30',
  'hover:scale-120 hover:shadow-md hover:shadow-primary/40',
  props.disabled && 'cursor-not-allowed',
]);

const formatValue = value => {
  return props.formatter(value);
};

const onInput = event => {
  localValue.value = Number(event.target.value);
  emit('update:modelValue', localValue.value);
  emit('input', localValue.value);
};

const onChange = event => {
  emit('change', Number(event.target.value));
};

const onTrackClick = event => {
  if (props.disabled) return;

  const track = trackRef.value;
  const rect = track.getBoundingClientRect();
  const percentage = (event.clientX - rect.left) / rect.width;
  const newValue = props.min + (props.max - props.min) * percentage;

  localValue.value = Math.round(newValue / props.step) * props.step;
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
