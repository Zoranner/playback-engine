<template>
  <aside class="left-panel">
    <PanelEventList />
    <PanelEnvironmentInfo />
  </aside>
</template>

<script setup>
import PanelEventList from './PanelEventList.vue';
import PanelEnvironmentInfo from './PanelEnvironmentInfo.vue';
</script>

<style scoped>
.left-panel {
  width: var(--panel-width);
  display: flex;
  flex-direction: column;
  padding: var(--spacing-sm);
  gap: var(--spacing-sm);
}
</style> 