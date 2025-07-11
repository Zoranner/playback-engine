<template>
  <div class="flex items-center gap-md">
    <div
      class="flex h-9 w-9 items-center justify-center rounded-md border border-border bg-gradient-default text-text-accent shadow-sm"
    >
      <Icon name="heroicons:globe-asia-australia" size="28" />
    </div>
    <div class="flex flex-col gap-xs">
      <span class="text-title font-semibold leading-tight">综合复盘软件</span>
      <span class="text-caption leading-tight opacity-75">{{ displayProjectName }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';

// 使用工程状态管理
const { currentProject } = useProject();

// 显示的工程名称
const displayProjectName = computed(() => {
  return currentProject.value?.name || '未打开工程';
});
</script>
