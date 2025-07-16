<template>
  <div
    class="relative h-8 border-b border-border bg-background-primary transition-colors hover:bg-background-secondary"
  >
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
    <div
      class="absolute inset-0 cursor-pointer"
      @click="onTrackClick"
    />
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
