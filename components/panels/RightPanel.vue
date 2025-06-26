<template>
  <aside class="right-panel">
    <PanelCoords />
    <PanelTargetInfo />
  </aside>
</template>

<script setup>
import PanelCoords from './PanelCoords.vue'
import PanelTargetInfo from './PanelTargetInfo.vue'
</script>

<style scoped>
.right-panel {
  width: var(--panel-width);
  display: flex;
  flex-direction: column;
  padding: var(--spacing-sm);
  gap: var(--spacing-sm);
}
</style> 