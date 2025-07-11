<template>
  <aside class="flex h-full w-panel flex-col gap-sm p-sm">
    <PanelEventList class="h-full flex-1" />
    <PanelEnvironmentInfo class="flex-none" />
  </aside>
</template>

<script setup>
import PanelEventList from './PanelEventList.vue';
import PanelEnvironmentInfo from './PanelEnvironmentInfo.vue';
</script>
