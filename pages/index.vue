<template>
  <div class="flex h-full flex-col overflow-hidden">
    <!-- 顶部标题栏 -->
    <TopHeader />

    <!-- 主要内容区域 -->
    <main
      class="flex w-full flex-1 overflow-hidden"
      style="height: calc(100vh - var(--header-height) - var(--status-bar-height))"
    >
      <!-- 左侧面板 -->
      <LeftPanel />

      <!-- 主海图区域 -->
      <MapContainer />

      <!-- 右侧面板 -->
      <RightPanel />
    </main>
  </div>
</template>

<script setup>
import TopHeader from '~/components/business/header/TopHeader.vue';
import LeftPanel from '~/components/business/panels/LeftPanel.vue';
import MapContainer from '~/components/business/map/MapContainer.vue';
import RightPanel from '~/components/business/panels/RightPanel.vue';
</script>
