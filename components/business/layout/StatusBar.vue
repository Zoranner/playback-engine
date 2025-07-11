<template>
  <div
    class="flex h-status flex-shrink-0 select-none items-center justify-between border-t border-border bg-background-header px-md text-text-muted"
  >
    <!-- 左侧状态 -->
    <div class="flex items-center gap-lg">
      <div class="flex items-center gap-xs">
        <div class="flex items-center gap-xs">
          <div class="h-2 w-2 rounded-full bg-success shadow-sm shadow-success/50" />
          <span class="font-medium">系统运行正常</span>
        </div>
      </div>

      <div class="flex items-center gap-xs">
        <span class="font-mono text-text-accent">{{ connectionStatus }}</span>
      </div>
    </div>

    <!-- 中心坐标 -->
    <div class="flex flex-1 justify-center">
      <div class="flex items-center gap-xs whitespace-nowrap">
        <span class="text-text-muted">坐标:</span>
        <span class="font-mono text-text-accent">{{ coordinates }}</span>
      </div>
    </div>

    <!-- 右侧信息 -->
    <div class="flex items-center gap-lg">
      <div class="flex items-center gap-xs whitespace-nowrap">
        <span class="text-text-muted">缩放:</span>
        <span class="font-mono text-text-accent">{{ zoomLevel }}%</span>
      </div>

      <div class="flex items-center gap-xs whitespace-nowrap">
        <span class="text-text-muted">{{ currentTime }}</span>
      </div>

      <div
        class="flex cursor-nw-resize items-center gap-xs opacity-50 transition-opacity duration-fast hover:opacity-100"
      >
        <Icon name="heroicons:arrows-pointing-in" class="h-3 w-3" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';

const currentTime = ref('');
const coordinates = ref('116.3974, 39.9093');
const zoomLevel = ref(100);
const connectionStatus = ref('TCP:CONNECTED');

const updateTime = () => {
  const now = new Date();
  currentTime.value = now.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
};

let timeInterval;
onMounted(() => {
  updateTime();
  timeInterval = setInterval(updateTime, 1000);
});

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval);
  }
});
</script>
