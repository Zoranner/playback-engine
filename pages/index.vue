<template>
  <div class="flex h-full flex-col overflow-hidden">
    <!-- 顶部标题栏 -->
    <TopHeader />

    <!-- 主要内容区域 -->
    <main
      class="relative flex-1 overflow-hidden"
      style="height: calc(100vh - var(--header-height) - var(--status-bar-height))"
    >
      <!-- 海图区域（撑满整个主区域） -->
      <div class="absolute inset-0 z-0">
        <MapContainer />
      </div>

      <!-- 界面层（叠加在海图上，左中右三栏布局） -->
      <div class="pointer-events-none absolute inset-0 z-10 flex bg-blue-500/20">
        <!-- 左侧面板 -->
        <div class="pointer-events-auto h-full flex-none bg-green-500/20">
          <LeftPanel />
        </div>

        <!-- 中间区域：上下结构 -->
        <div class="flex h-full flex-1 flex-col bg-yellow-500/20">
          <!-- 上方空白区域，左上角工具，右上角图例 -->
          <div class="relative flex-1 bg-purple-500/20">
            <!-- 左上角工具条 -->
            <div class="absolute left-sm top-sm">
              <MapTools @zoom-in="zoomIn" @zoom-out="zoomOut" />
            </div>
            <!-- 右上角图例 -->
            <div class="pointer-events-auto absolute right-sm top-sm">
              <MapLegend />
            </div>
          </div>
          <!-- 下方回放控制 -->
          <div
            class="pointer-events-auto flex-none border-t border-border bg-background-secondary p-md"
          >
            <TimelinePlayer />
          </div>
        </div>

        <!-- 右侧面板 -->
        <div class="pointer-events-auto h-full flex-none bg-green-500/20">
          <RightPanel />
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import TopHeader from '~/components/business/header/TopHeader.vue';
import LeftPanel from '~/components/business/panels/LeftPanel.vue';
import MapContainer from '~/components/business/map/MapContainer.vue';
import RightPanel from '~/components/business/panels/RightPanel.vue';
import TimelinePlayer from '~/components/business/timeline/TimelinePlayer.vue';
import MapLegend from '~/components/business/map/MapLegend.vue';
import MapTools from '~/components/business/map/MapTools.vue';
</script>
