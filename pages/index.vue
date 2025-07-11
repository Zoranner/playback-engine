<template>
  <!-- 主要内容区域 -->
  <main class="relative flex-1 overflow-hidden">
    <!-- 海图区域（撑满整个主区域） -->
    <MapContainer class="absolute inset-0 z-0" />

    <!-- 界面层（叠加在海图上） -->
    <InterfaceContainer class="absolute inset-0 z-10" />
  </main>
</template>

<script setup>
import MapContainer from '~/components/business/features/map/MapContainer.vue';
import InterfaceContainer from '~/components/business/containers/InterfaceContainer.vue';
</script>
