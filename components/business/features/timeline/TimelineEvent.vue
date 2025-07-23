<template>
  <div
    v-if="isVisible"
    :class="eventClasses"
    :style="eventStyles"
    @mouseenter="showTooltip = true"
    @mouseleave="showTooltip = false"
  >
    <!-- 悬停提示 -->
    <div
      v-if="showTooltip"
      class="pointer-events-none absolute bottom-full left-1/2 z-50 mb-1 -translate-x-1/2 transform whitespace-nowrap rounded border border-border bg-background-primary px-2 py-1 text-xs shadow-lg"
    >
      <div class="font-medium text-text-primary">
        {{ event.title }}
      </div>
      <div
        v-if="event.description"
        class="mt-0.5 text-text-secondary"
      >
        {{ event.description }}
      </div>
      <div class="text-text-tertiary mt-0.5">
        {{ formatTimeSmart(event.startTime, totalDuration) }}
        <span v-if="event.endTime">
          -
          {{ formatTimeSmart(event.endTime, totalDuration) }}</span
        >
      </div>

      <!-- 提示箭头 -->
      <div class="absolute left-1/2 top-full -mt-px -translate-x-1/2 transform">
        <div
          class="h-2 w-2 rotate-45 transform border-b border-r border-border bg-background-primary"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { TimelineEvent } from '~/composables/useTimeline';

interface Props {
  event: TimelineEvent;
  totalDuration: number;
}

const props = defineProps<Props>();

const { formatTime, formatTimeSmart } = useTimeline();

const showTooltip = ref(false);

// 事件类型颜色映射
const eventTypeColors = {
  success: {
    bg: 'bg-success',
    border: 'border-success-dark',
  },
  warning: {
    bg: 'bg-warning',
    border: 'border-warning-dark',
  },
  error: {
    bg: 'bg-danger',
    border: 'border-danger-dark',
  },
  info: {
    bg: 'bg-primary',
    border: 'border-primary-dark',
  },
};

// 计算事件是否可见
const isVisible = computed(() => {
  return props.totalDuration > 0;
});

// 计算事件的样式类
const eventClasses = computed(() => {
  const colors = eventTypeColors[props.event.type] || eventTypeColors.info;
  const isRangeEvent = !!props.event.endTime;

  return [
    'absolute top-0 cursor-pointer transition-all duration-200 hover:z-10',
    colors.bg,
    colors.border,
    'border-2',
    isRangeEvent ? 'h-full rounded-sm' : 'h-full w-2 rounded-sm',
    'hover:brightness-110',
    'hover:scale-105',
    'shadow-sm',
  ];
});

// 计算事件的位置和尺寸样式
const eventStyles = computed(() => {
  if (props.totalDuration === 0) return {};

  const startPercent = (props.event.startTime / props.totalDuration) * 100;

  if (props.event.endTime) {
    // 区间事件
    const endPercent = (props.event.endTime / props.totalDuration) * 100;
    const width = endPercent - startPercent;

    return {
      left: `${startPercent}%`,
      width: `${Math.max(width, 0.1)}%`, // 最小宽度确保可见
    };
  } else {
    // 点事件
    return {
      left: `${startPercent}%`,
      transform: 'translateX(-50%)', // 居中对齐
    };
  }
});
</script>
