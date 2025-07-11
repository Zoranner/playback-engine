<template>
  <aside class="flex w-panel flex-col gap-sm">
    <PanelCoords class="flex-none" />
    <PanelTargetInfo class="h-full flex-1" />
  </aside>
</template>

<script setup>
import PanelCoords from './PanelCoords.vue';
import PanelTargetInfo from './PanelTargetInfo.vue';
</script>
