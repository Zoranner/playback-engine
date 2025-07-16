<template>
  <GroupBox
    title="事件列表"
    class="flex flex-1 flex-col overflow-hidden"
  >
    <div class="-m-sm flex flex-1 flex-col gap-xs overflow-y-auto p-sm">
      <div
        v-for="event in events"
        :key="event.id"
        :class="getEventItemClasses(event)"
        @click="selectEvent(event.id)"
      >
        <div :class="getIndicatorClasses(event.level)" />
        <div class="text-monospace mt-px min-w-12">{{ event.time }}</div>
        <div class="min-w-0 flex-1">
          <div class="mb-0.5 text-subtitle">{{ event.title }}</div>
          <div class="text-caption leading-snug">
            {{ event.description }}
          </div>
        </div>
      </div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref } from 'vue';
import { useEvents } from '~/composables/useEvents';
import GroupBox from '~/components/display/GroupBox.vue';

const { events } = useEvents();
const selectedEvent = ref(null);

// 事件项样式类
const getEventItemClasses = event => {
  const baseClasses = [
    'flex items-start gap-sm px-sm py-xs bg-background-tertiary border border-border rounded-sm',
    'cursor-pointer transition-all duration-fast relative',
  ];

  // 左边框颜色
  const levelBorders = {
    high: 'border-l-3 border-l-danger',
    warning: 'border-l-3 border-l-warning',
    normal: 'border-l-3 border-l-success',
    low: 'border-l-3 border-l-text-muted',
  };

  baseClasses.push(levelBorders[event.level] || levelBorders.normal);

  if (selectedEvent.value === event.id) {
    baseClasses.push('bg-background-panel border-border-active shadow-glow');
  } else {
    baseClasses.push('hover:bg-background-panel hover:border-border-light hover:translate-x-0.5');
  }

  return baseClasses;
};

// 指示器样式类
const getIndicatorClasses = level => {
  const baseClasses = 'w-3 h-3 rounded-full mt-0.5 flex-shrink-0 shadow-sm';

  const levelVariants = {
    high: 'bg-danger shadow-danger/30',
    warning: 'bg-warning shadow-warning/30',
    normal: 'bg-success shadow-success/30',
    low: 'bg-text-muted shadow-text-muted/30',
  };

  return [baseClasses, levelVariants[level] || levelVariants.normal];
};

const selectEvent = eventId => {
  selectedEvent.value = selectedEvent.value === eventId ? null : eventId;
};
</script>

<!-- 完全移除 <style> 标签 -->
