<template>
  <div class="map-tools">
    <!-- 缩放按钮组 - 左上角 -->
    <div class="zoom-controls">
      <button 
        class="action-btn" 
        title="放大"
        @click="zoomIn"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <circle cx="11" cy="11" r="8"/>
          <path d="21 21l-4.35-4.35"/>
          <line x1="11" y1="8" x2="11" y2="14"/>
          <line x1="8" y1="11" x2="14" y2="11"/>
        </svg>
      </button>
      
      <button 
        class="action-btn" 
        title="缩小"
        @click="zoomOut"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <circle cx="11" cy="11" r="8"/>
          <path d="21 21l-4.35-4.35"/>
          <line x1="8" y1="11" x2="14" y2="11"/>
        </svg>
      </button>
      
      <button 
        class="action-btn" 
        title="重置缩放"
        @click="resetZoom"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
          <line x1="9" y1="9" x2="15" y2="15"/>
          <line x1="15" y1="9" x2="9" y2="15"/>
        </svg>
      </button>
    </div>

    <!-- 测距工具 - 右上角 -->
    <div class="measure-tool">
      <button 
        class="tool-btn" 
        :class="{ 'tool-btn--active': activeTool === 'measure-distance' }"
        @click="setActiveTool('measure-distance')"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor">
          <path d="M16 4l4 4-4 4"/>
          <path d="M20 8H4"/>
          <path d="M8 20l-4-4 4-4"/>
        </svg>
        <span class="tool-label">测距</span>
      </button>
    </div>

    <!-- 测量结果 -->
    <div v-if="measurementResult" class="measurement-result">
      <div class="measurement-content">
        <div class="measurement-title">{{ measurementResult.type }}</div>
        <div class="measurement-value text-monospace">{{ measurementResult.value }}</div>
        <button class="clear-btn" title="清除" @click="clearMeasurement">×</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

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

/* 动作按钮样式 */
.action-btn {
  width: 24px;
  height: 24px;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all var(--transition-fast);
  padding: 0;
  backdrop-filter: blur(4px);
  box-shadow: var(--shadow-sm);
}

.action-btn:hover {
  background: var(--tertiary-bg);
  border-color: var(--border-color-light);
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.action-btn:active {
  transform: translateY(0);
  box-shadow: var(--shadow-inset);
}

/* 工具按钮样式 - 带文字 */
.tool-btn {
  height: 24px;
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  cursor: pointer;
  transition: all var(--transition-fast);
  padding: 0 var(--spacing-sm);
  backdrop-filter: blur(4px);
  box-shadow: var(--shadow-sm);
}

.tool-btn:hover {
  background: linear-gradient(145deg, var(--border-color-light), var(--tertiary-bg));
  border-color: var(--border-color-light);
  color: var(--text-primary);
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.tool-btn--active {
  background: linear-gradient(145deg, #0ea5e9, #0284c7);
  border-color: #0ea5e9;
  color: white;
  box-shadow: var(--glow-subtle);
}

.tool-btn--active:hover {
  background: linear-gradient(145deg, #38bdf8, #0ea5e9);
}

.tool-label {
  font-size: 12px;
  font-weight: 500;
  white-space: nowrap;
}

.tool-btn svg,
.action-btn svg {
  stroke-width: 1.8;
  flex-shrink: 0;
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

.clear-btn {
  position: absolute;
  top: 2px;
  right: 2px;
  width: 16px;
  height: 16px;
  background: var(--danger-color);
  border: none;
  border-radius: 50%;
  color: white;
  font-size: 12px;
  line-height: 1;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.clear-btn:hover {
  transform: scale(1.1);
}
</style> 