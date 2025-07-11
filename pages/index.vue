<template>
  <!-- 主要内容区域 -->
  <main class="relative flex-1 overflow-hidden">
    <!-- 海图区域（撑满整个主区域） -->
    <MapContainer class="absolute inset-0 z-0" />

    <!-- 界面层（叠加在海图上） -->
    <InterfaceLayer class="absolute inset-0 z-10" />
  </main>
</template>

<script setup>
import MapContainer from '~/components/business/layout/MapContainer.vue';
import InterfaceLayer from '~/components/business/layout/InterfaceLayer.vue';
</script>
