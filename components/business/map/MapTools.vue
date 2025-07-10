<template>
  <div class="pointer-events-none absolute inset-0 z-30">
    <!-- 缩放按钮组 - 左上角 -->
    <div class="pointer-events-auto absolute left-sm top-sm flex flex-col gap-px">
      <Button square title="放大" icon="heroicons:magnifying-glass-plus" @click="zoomIn" />

      <Button square title="缩小" icon="heroicons:magnifying-glass-minus" @click="zoomOut" />

      <Button square title="重置缩放" icon="heroicons:arrows-pointing-out" @click="resetZoom" />
    </div>

    <!-- 测距工具 - 右上角 -->
    <div class="pointer-events-auto absolute right-sm top-sm">
      <Button
        icon="heroicons:arrows-right-left"
        :active="activeTool === 'measure-distance'"
        @click="setActiveTool('measure-distance')"
      >
        测距
      </Button>
    </div>

    <!-- 测量结果 -->
    <div
      v-if="measurementResult"
      class="min-w-30 pointer-events-auto absolute left-sm top-[calc(theme(spacing.sm)+32px)] rounded-sm border border-warning bg-background-panel shadow-md backdrop-blur-sm"
    >
      <div class="relative p-xs">
        <div class="mb-0.5 font-semibold text-text-secondary">{{ measurementResult.type }}</div>
        <div class="text-monospace font-semibold text-warning">{{ measurementResult.value }}</div>
        <Button
          square
          variant="danger"
          title="清除"
          icon="heroicons:x-mark"
          class="absolute right-1 top-1"
          @click="clearMeasurement"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import Button from '~/components/base/Button.vue';

// 工具状态
const activeTool = ref('select');

// 测量结果
const measurementResult = ref(null);

// 动作按钮事件
const zoomIn = () => {
  emit('zoom-in');
};

const zoomOut = () => {
  emit('zoom-out');
};

const resetZoom = () => {
  emit('reset-zoom');
};

// 工具状态切换
const setActiveTool = tool => {
  if (activeTool.value === tool) {
    // 如果点击的是当前激活的工具，取消激活
    activeTool.value = 'select';
    emit('tool-changed', 'select');
  } else {
    activeTool.value = tool;
    emit('tool-changed', tool);
  }
};

// 测量相关
const clearMeasurement = () => {
  measurementResult.value = null;
  emit('clear-measurement');
};

// 对外暴露的方法
const setMeasurementResult = result => {
  measurementResult.value = result;
};

// 定义事件
const emit = defineEmits([
  'zoom-in',
  'zoom-out',
  'reset-zoom',
  'tool-changed',
  'clear-measurement',
]);

// 暴露方法给父组件
defineExpose({
  setMeasurementResult,
  setActiveTool,
});
</script>

<!-- 完全移除 <style> 标签 -->
