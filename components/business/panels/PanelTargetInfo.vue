<template>
  <GroupBox title="目标信息" class="flex flex-1 flex-col overflow-hidden">
    <div class="-m-sm flex flex-1 flex-col gap-xs overflow-y-auto p-sm">
      <div
        v-for="target in targets"
        :key="target.id"
        :class="targetItemClasses(target)"
        @click="handleSelectTarget(target)"
      >
        <div
          class="h-2.5 w-2.5 flex-shrink-0 rounded-full border border-border-light shadow-sm"
          :style="{ background: target.color }"
        />
        <div class="min-w-0 flex-1">
          <div class="mb-0.5 text-subtitle font-semibold">{{ target.label }}</div>
          <div class="text-monospace mb-0.5 opacity-70">{{ target.id }}</div>
          <div class="mb-px flex items-center">
            <span class="min-w-8 text-caption">距离: </span>
            <span class="text-monospace">{{ target.distance }}海里</span>
          </div>
          <div class="mb-px flex items-center">
            <span class="min-w-8 text-caption">方位: </span>
            <span class="text-monospace">{{ target.bearing }}°</span>
          </div>
        </div>
        <div :class="getStatusClasses(target.status)" />
      </div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref, computed } from 'vue';
import { useTargets } from '~/composables/useTargets';
import GroupBox from '~/components/display/GroupBox.vue';

const { targets, selectTarget } = useTargets();
const selectedTarget = ref(null);

// 目标项样式类
const targetItemClasses = target => {
  const baseClasses = [
    'flex items-center gap-sm p-sm bg-background-tertiary border border-border rounded-sm',
    'cursor-pointer transition-all duration-fast relative',
  ];

  if (selectedTarget.value === target.id) {
    baseClasses.push('bg-background-panel border-border-active shadow-glow');
  } else {
    baseClasses.push(
      'hover:bg-background-panel hover:border-border-light hover:-translate-y-px hover:shadow-sm'
    );
  }

  return baseClasses;
};

// 状态指示器样式类
const getStatusClasses = status => {
  const baseClasses = 'w-2 h-2 rounded-full flex-shrink-0 shadow-sm';

  const statusVariants = {
    friendly: 'bg-success shadow-success/30',
    neutral: 'bg-warning shadow-warning/30',
    unknown: 'bg-danger shadow-danger/30',
  };

  return [baseClasses, statusVariants[status] || statusVariants.unknown];
};

const handleSelectTarget = target => {
  selectedTarget.value = selectedTarget.value === target.id ? null : target.id;
  selectTarget(target);
};
</script>
