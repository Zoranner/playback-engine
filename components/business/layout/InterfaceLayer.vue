<template>
  <!-- 界面层（左中右三栏布局） -->
  <div class="pointer-events-none flex h-full gap-sm p-sm">
    <!-- 左侧面板 -->
    <LeftPanel class="pointer-events-auto h-full flex-none" />

    <!-- 中间区域：上下结构 -->
    <div class="flex h-full flex-1 flex-col gap-sm">
      <!-- 上方空白区域，左上角工具，右上角图例 -->
      <div class="flex flex-1 justify-between">
        <!-- 左上角工具 -->
        <div class="pointer-events-auto flex-none self-start">
          <MapTools />
        </div>
        <!-- 右上角图例 -->
        <div class="pointer-events-auto flex-none self-start">
          <MapLegend />
        </div>
      </div>
      <!-- 下方回放控制 -->
      <TimelinePlayer class="pointer-events-auto flex-none" />
    </div>

    <!-- 右侧面板 -->
    <RightPanel class="pointer-events-auto h-full flex-none" />
  </div>
</template>

<script setup>
import LeftPanel from '~/components/business/features/panels/LeftPanel.vue';
import RightPanel from '~/components/business/features/panels/RightPanel.vue';
import TimelinePlayer from '~/components/business/features/timeline/TimelinePlayer.vue';
import MapLegend from '~/components/business/features/map/MapLegend.vue';
import MapTools from '~/components/business/features/map/MapTools.vue';
</script>
