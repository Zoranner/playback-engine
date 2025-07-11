<template>
  <aside class="flex w-panel flex-col gap-sm">
    <PanelEventList class="h-full flex-1" />
    <PanelEnvironmentInfo class="flex-none" />
  </aside>
</template>

<script setup>
import PanelEventList from './PanelEventList.vue';
import PanelEnvironmentInfo from './PanelEnvironmentInfo.vue';
</script>
