<template>
  <div class="map-tools">
    <!-- 缩放按钮组 - 左上角 -->
    <div class="zoom-controls">
      <Button 
        size="small"
        square
        title="放大"
        @click="zoomIn"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <circle cx="11" cy="11" r="8"/>
          <path d="21 21l-4.35-4.35"/>
          <line x1="11" y1="8" x2="11" y2="14"/>
          <line x1="8" y1="11" x2="14" y2="11"/>
        </svg>
      </Button>
      
      <Button 
        size="small"
        square
        title="缩小"
        @click="zoomOut"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <circle cx="11" cy="11" r="8"/>
          <path d="21 21l-4.35-4.35"/>
          <line x1="8" y1="11" x2="14" y2="11"/>
        </svg>
      </Button>
      
      <Button 
        size="small"
        square
        title="重置缩放"
        @click="resetZoom"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
          <line x1="9" y1="9" x2="15" y2="15"/>
          <line x1="15" y1="9" x2="9" y2="15"/>
        </svg>
      </Button>
    </div>

    <!-- 测距工具 - 右上角 -->
    <div class="measure-tool">
      <Button 
        size="small"
        :active="activeTool === 'measure-distance'"
        @click="setActiveTool('measure-distance')"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <path d="M16 4l4 4-4 4"/>
          <path d="M20 8H4"/>
          <path d="M8 20l-4-4 4-4"/>
        </svg>
        <span class="tool-label">测距</span>
      </Button>
    </div>

    <!-- 测量结果 -->
    <div v-if="measurementResult" class="measurement-result">
      <div class="measurement-content">
        <div class="measurement-title">{{ measurementResult.type }}</div>
        <div class="measurement-value text-monospace">{{ measurementResult.value }}</div>
        <Button size="small" square variant="danger" title="清除" @click="clearMeasurement">×</Button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import Button from '~/components/ui/Button.vue'

// 工具状态
const activeTool = ref('select')

// 测量结果
const measurementResult = ref(null)

// 动作按钮事件
const zoomIn = () => {
  emit('zoom-in')
}

const zoomOut = () => {
  emit('zoom-out')
}

const resetZoom = () => {
  emit('reset-zoom')
}

// 工具状态切换
const setActiveTool = (tool) => {
  if (activeTool.value === tool) {
    // 如果点击的是当前激活的工具，取消激活
    activeTool.value = 'select'
    emit('tool-changed', 'select')
  } else {
    activeTool.value = tool
    emit('tool-changed', tool)
  }
}

// 测量相关
const clearMeasurement = () => {
  measurementResult.value = null
  emit('clear-measurement')
}

// 对外暴露的方法
const setMeasurementResult = (result) => {
  measurementResult.value = result
}

// 定义事件
const emit = defineEmits([
  'zoom-in',
  'zoom-out', 
  'reset-zoom',
  'tool-changed',
  'clear-measurement'
])

// 暴露方法给父组件
defineExpose({
  setMeasurementResult,
  setActiveTool
})
</script>

<style scoped>
.map-tools {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  pointer-events: none; /* 允许点击穿透 */
  z-index: 30;
}

/* 缩放控制 - 左上角 */
.zoom-controls {
  position: absolute;
  top: var(--spacing-sm);
  left: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: 1px;
  pointer-events: auto;
}

/* 测距工具 - 右上角 */
.measure-tool {
  position: absolute;
  top: var(--spacing-sm);
  right: var(--spacing-sm);
  pointer-events: auto;
}

/* 工具相关样式 */
.tool-label {
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
}

.measurement-result {
  position: absolute;
  top: calc(var(--spacing-sm) + 32px);
  left: var(--spacing-sm);
  background: var(--panel-bg);
  border: 1px solid var(--warning-color);
  border-radius: var(--radius-sm);
  backdrop-filter: blur(4px);
  box-shadow: var(--shadow-md);
  min-width: 120px;
  pointer-events: auto;
}

.measurement-content {
  padding: var(--spacing-xs);
  position: relative;
}

.measurement-title {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 2px;
}

.measurement-value {
  font-size: 12px;
  color: var(--warning-color);
  font-weight: 600;
}

/* 清除按钮现在使用Button组件 */
</style> 