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
        v-if="showTime"
        class="min-w-8 whitespace-nowrap text-center text-text-secondary"
      >
        {{ startTime }}
      </span>

      <div
        ref="trackRef"
        :class="trackClasses"
        @click="onTrackClick"
      >
        <!-- 背景轨道 -->
        <div class="h-1 w-full rounded-sm border border-border bg-background-secondary" />

        <!-- 进度填充 -->
        <div
          :class="fillClasses"
          :style="{ width: progressPercentage + '%' }"
        />

        <!-- 拖拽点 -->
        <div
          v-if="showThumb"
          :class="thumbClasses"
          :style="{ left: progressPercentage + '%' }"
        />
      </div>

      <span
        v-if="showTime"
        class="min-w-8 whitespace-nowrap text-center text-text-secondary"
      >
        {{ currentTime }}
      </span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  // 当前值
  modelValue: {
    type: Number,
    required: true,
  },
  // 最大值
  max: {
    type: Number,
    default: 100,
  },
  // 最小值
  min: {
    type: Number,
    default: 0,
  },
  // 标签
  label: {
    type: String,
    default: '',
  },
  // 是否可点击
  clickable: {
    type: Boolean,
    default: false,
  },
  // 是否显示拖拽点
  showThumb: {
    type: Boolean,
    default: false,
  },
  // 是否显示时间
  showTime: {
    type: Boolean,
    default: false,
  },
  // 开始时间
  startTime: {
    type: String,
    default: '00:00',
  },
  // 当前时间
  currentTime: {
    type: String,
    default: '00:00',
  },
  // 变体
  variant: {
    type: String,
    default: 'primary',
    validator: value => ['primary', 'success', 'warning', 'danger'].includes(value),
  },
});

const emit = defineEmits(['update:modelValue', 'change']);

const trackRef = ref(null);

const progressPercentage = computed(() => {
  const range = props.max - props.min;
  if (range === 0) return 0;
  return Math.max(0, Math.min(100, ((props.modelValue - props.min) / range) * 100));
});

// 轨道样式类
const trackClasses = computed(() => [
  'relative flex-1 min-w-25',
  props.clickable && 'cursor-pointer',
]);

// 填充样式类
const fillClasses = computed(() => {
  const baseClasses = [
    'absolute top-0 left-0 h-1 rounded-sm transition-[width] duration-fast z-10',
  ];

  const variantClasses = {
    primary: ['bg-gradient-to-r from-info to-text-accent', 'shadow-sm shadow-info/30'],
    success: ['bg-gradient-to-r from-success to-success-light', 'shadow-sm shadow-success/30'],
    warning: ['bg-gradient-to-r from-warning to-warning-light', 'shadow-sm shadow-warning/30'],
    danger: ['bg-gradient-to-r from-danger to-danger-light', 'shadow-sm shadow-danger/30'],
  };

  return [...baseClasses, ...(variantClasses[props.variant] ?? variantClasses.primary)];
});

// 拖拽点样式类
const thumbClasses = computed(() => [
  'absolute top-1/2 w-2 h-2 bg-text-accent border-2 border-background-primary',
  'rounded-full transform -translate-x-1/2 -translate-y-1/2 cursor-pointer',
  'transition-all duration-fast z-20 shadow-sm shadow-primary/30',
  'hover:scale-120 hover:shadow-md hover:shadow-primary/40',
]);

const onTrackClick = event => {
  if (!props.clickable) return;

  const track = trackRef.value;
  const rect = track.getBoundingClientRect();
  const percentage = Math.max(0, Math.min(1, (event.clientX - rect.left) / rect.width));
  const newValue = props.min + (props.max - props.min) * percentage;

  emit('update:modelValue', newValue);
  emit('change', newValue);
};
</script>
