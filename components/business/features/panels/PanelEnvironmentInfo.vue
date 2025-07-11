<template>
  <GroupBox title="环境信息" class="flex-none">
    <div class="-m-sm flex flex-col gap-xs p-sm">
      <div
        v-for="item in environmentData"
        :key="item.label"
        class="flex items-center justify-between rounded-sm border border-border bg-background-tertiary px-sm py-xs transition-all duration-fast hover:border-border-light hover:bg-background-panel"
      >
        <span class="text-caption">{{ item.label }}</span>
        <span class="text-monospace font-semibold">{{ item.value }}</span>
      </div>
    </div>
  </GroupBox>
</template>

<script setup>
import { useEnvironment } from '~/composables/useEnvironment';
import GroupBox from '~/components/display/GroupBox.vue';

const { environmentData } = useEnvironment();
</script>

<!-- 完全移除 <style> 标签 -->
