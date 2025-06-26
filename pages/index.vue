<template>
  <div class="page-container">
    <!-- 顶部标题栏 -->
    <TopHeader />

    <!-- 主要内容区域 -->
    <main class="main-content">
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
import TopHeader from '~/components/header/TopHeader.vue'
import LeftPanel from '~/components/panels/LeftPanel.vue'
import MapContainer from '~/components/map/MapContainer.vue'
import RightPanel from '~/components/panels/RightPanel.vue'
</script>

<style scoped>
.page-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  height: calc(100vh - var(--header-height) - var(--status-bar-height));
  width: 100%;
}
</style> 