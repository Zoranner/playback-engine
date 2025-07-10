<template>
  <div
    class="flex h-10 items-center justify-between border-b border-border bg-background-secondary px-md"
  >
    <!-- 左侧：播放控制区域 -->
    <div class="flex items-center gap-sm">
      <!-- 播放/暂停按钮 -->
      <Button
        :icon="isPlaying ? 'heroicons:pause' : 'heroicons:play'"
        :variant="isPlaying ? 'success' : 'primary'"
        :disabled="!canControl"
        square
        @click="togglePlayPause"
      />

      <!-- 停止按钮 -->
      <Button icon="heroicons:stop" variant="danger" :disabled="!canControl" square @click="stop" />

      <!-- 分隔线 -->
      <div class="h-6 w-px bg-border-light" />

      <!-- 倍速控制 -->
      <div class="flex items-center gap-xs">
        <span class="text-sm text-text-secondary">倍速</span>
        <Select
          :model-value="currentSpeedOption"
          :options="speedOptions"
          variant="default"
          size="small"
          @update:model-value="onSpeedChange"
        />
      </div>
    </div>

    <!-- 中间：时间显示 -->
    <div class="flex items-center gap-md">
      <div class="flex items-center gap-xs font-mono text-sm">
        <span class="text-text-primary">{{ currentTimeString }}</span>
        <span class="text-text-secondary">/</span>
        <span class="text-text-secondary">{{ totalDurationString }}</span>
      </div>
    </div>

    <!-- 右侧：控制按钮 -->
    <div class="flex items-center gap-xs">
      <!-- 测试数据按钮 -->
      <Button v-if="totalDuration === 0" variant="primary" size="small" @click="loadTestData">
        加载测试数据
      </Button>

      <!-- 缩放控制 -->
      <Button
        icon="heroicons:magnifying-glass-minus"
        variant="ghost"
        size="small"
        square
        :disabled="zoomLevel <= 0.1"
        @click="zoomOut"
      />
      <span class="px-xs text-xs text-text-secondary">{{ Math.round(zoomLevel * 100) }}%</span>
      <Button
        icon="heroicons:magnifying-glass-plus"
        variant="ghost"
        size="small"
        square
        :disabled="zoomLevel >= 10"
        @click="zoomIn"
      />

      <!-- 分隔线 -->
      <div class="h-6 w-px bg-border-light" />

      <!-- 展开/收起按钮 -->
      <Button
        :icon="isExpanded ? 'heroicons:chevron-up' : 'heroicons:chevron-down'"
        variant="ghost"
        size="small"
        square
        :active="isExpanded"
        @click="toggleExpanded"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick } from 'vue';
import Button from '~/components/base/Button.vue';
import Select from '~/components/input/Select.vue';

const {
  // 状态
  isPlaying,
  currentTimeString,
  totalDurationString,
  playbackSpeed,
  speedOptions,
  isExpanded,
  zoomLevel,
  totalDuration,
  platforms,

  // 方法
  togglePlayPause,
  stop,
  setPlaybackSpeed,
  toggleExpanded,
  setZoom,
  initializeTestData,
} = useTimeline();

// 是否可以控制（有项目且总时长大于0）
const canControl = computed(() => totalDuration.value > 0);

// 当前倍速选项
const currentSpeedOption = computed(() => {
  return speedOptions.find(option => option.value === playbackSpeed.value) || speedOptions[0];
});

// 倍速变化处理
const onSpeedChange = (option: { value: number; label: string }) => {
  setPlaybackSpeed(option.value);
};

// 缩放控制
const zoomIn = () => {
  setZoom(zoomLevel.value * 1.2);
};

const zoomOut = () => {
  setZoom(zoomLevel.value / 1.2);
};

// 加载测试数据
const loadTestData = async () => {
  initializeTestData();
  console.log('从时间轴头部加载测试数据');

  // 等待下一个tick确保DOM更新
  await nextTick();
  console.log('数据更新完成，平台数量:', platforms?.value?.length || 0);
};
</script>
