<template>
  <div
    class="flex h-8 border-b border-border bg-background-primary transition-colors hover:bg-background-secondary"
  >
    <!-- 左侧：平台名称 -->
    <div class="flex w-20 items-center border-r border-border bg-background-tertiary px-xs">
      <span class="truncate text-xs font-medium text-text-primary">{{ platform.name }}</span>
    </div>

    <!-- 中间：时间轴区域 -->
    <div class="relative flex-1 overflow-hidden">
      <!-- 背景（非活动区间） -->
      <div class="absolute inset-0 bg-background-secondary" />

      <!-- 活动区间 -->
      <div
        v-for="(segment, index) in activeSegments"
        :key="`segment-${index}`"
        class="absolute top-0 h-full border border-success-dark bg-success opacity-70 transition-all duration-300"
        :style="getSegmentStyle(segment)"
      />

      <!-- 事件标记 -->
      <div class="absolute inset-0">
        <TimelineEvent
          v-for="event in platform.events"
          :key="event.id"
          :event="event"
          :total-duration="totalDuration"
        />
      </div>

      <!-- 点击区域（用于跳转时间） -->
      <div class="absolute inset-0 cursor-pointer" @click="onTrackClick" />
    </div>

    <!-- 右侧：状态指示器 -->
    <div class="flex items-center gap-xs border-l border-border bg-background-tertiary px-xs">
      <!-- 活动状态指示器 -->
      <div
        :class="['h-2 w-2 rounded-full', platform.isActive ? 'bg-success' : 'bg-border']"
        :title="platform.isActive ? '平台活动' : '平台非活动'"
      />

      <!-- 事件计数器（如果有事件） -->
      <div v-if="platform.events.length > 0" class="flex items-center gap-xs">
        <!-- 警告图标 -->
        <div
          v-if="warningCount > 0"
          class="flex items-center gap-xs"
          :title="`${warningCount}个警告`"
        >
          <Icon name="heroicons:exclamation-triangle" class="h-3 w-3 text-warning" />
          <span class="text-xs text-warning">{{ warningCount }}</span>
        </div>

        <!-- 错误图标 -->
        <div v-if="errorCount > 0" class="flex items-center gap-xs" :title="`${errorCount}个错误`">
          <Icon name="heroicons:x-circle" class="h-3 w-3 text-danger" />
          <span class="text-xs text-danger">{{ errorCount }}</span>
        </div>
      </div>

      <!-- 平台类型标识 -->
      <div class="text-text-tertiary text-xs">
        {{ getPlatformTypeLabel() }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Platform } from '~/composables/useTimeline';
import TimelineEvent from './TimelineEvent.vue';

interface Props {
  platform: Platform;
}

const props = defineProps<Props>();

const { totalDuration, seekTo } = useTimeline();

// 计算活动区间样式
const activeSegments = computed(() => {
  return props.platform.activeSegments || [];
});

// 获取区间样式
const getSegmentStyle = (segment: { startTime: number; endTime: number }) => {
  if (totalDuration.value === 0) return { display: 'none' };

  const startPercent = (segment.startTime / totalDuration.value) * 100;
  const endPercent = (segment.endTime / totalDuration.value) * 100;
  const width = endPercent - startPercent;

  return {
    left: `${startPercent}%`,
    width: `${Math.max(width, 0.1)}%`,
  };
};

// 事件计数
const warningCount = computed(() => {
  return props.platform.events.filter(event => event.type === 'warning').length;
});

const errorCount = computed(() => {
  return props.platform.events.filter(event => event.type === 'error').length;
});

// 获取平台类型标签
const getPlatformTypeLabel = () => {
  // 根据平台ID推断类型
  const id = props.platform.id;
  if (id.includes('ITC')) return 'ITC';
  if (id.includes('X1')) return 'X1';
  if (id.includes('Y-')) return 'Y';
  return 'A';
};

// 轨道点击事件
const onTrackClick = (event: MouseEvent) => {
  console.log(`点击了平台轨道: ${props.platform.name}`);
  if (totalDuration.value === 0) return;

  const target = event.currentTarget as HTMLElement;
  const rect = target.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const percentage = (x / rect.width) * 100;
  const time = (percentage / 100) * totalDuration.value;

  console.log(`${props.platform.name} 点击位置: ${percentage.toFixed(1)}%, 跳转到: ${time}ms`);
  seekTo(time);
};
</script>
