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

      <!-- 中间区域：地图 + 时间轴 -->
      <div class="flex flex-1 flex-col overflow-hidden">
        <!-- 主海图区域 -->
        <div class="flex-1 overflow-hidden">
          <MapContainer />
        </div>

        <!-- 时间轴播放器 -->
        <div class="border-t border-border bg-background-secondary p-md">
          <TimelinePlayer />
        </div>
      </div>

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
import TimelinePlayer from '~/components/business/timeline/TimelinePlayer.vue';
</script>
