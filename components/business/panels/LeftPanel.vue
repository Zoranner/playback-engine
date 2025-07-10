<template>
  <aside class="flex w-panel flex-col gap-sm p-sm">
    <PanelEventList />
    <PanelEnvironmentInfo />
  </aside>
</template>

<script setup>
import PanelEventList from './PanelEventList.vue';
import PanelEnvironmentInfo from './PanelEnvironmentInfo.vue';
</script>
