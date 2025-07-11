<template>
  <aside class="flex h-full w-panel flex-col gap-sm p-sm">
    <PanelCoords class="flex-none" />
    <PanelTargetInfo class="h-full flex-1" />
  </aside>
</template>

<script setup>
import PanelCoords from './PanelCoords.vue';
import PanelTargetInfo from './PanelTargetInfo.vue';
</script>
