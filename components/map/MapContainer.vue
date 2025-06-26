<template>
  <div class="map-container">
    <!-- 主要地图区域 -->
    <MapMainArea 
      ref="mapMainArea"
      :active-tool="activeTool"
      :visible-layers="visibleLayers"
      @coordinates-changed="updateCoordinates"
      @zoom-changed="updateZoom"
      @bearing-changed="updateBearing"
      @measurement-result="updateMeasurement"
      @equipment-selected="handleEquipmentSelected"
    />
    
    <!-- 地图工具 - 左上角 -->
    <MapTools 
      ref="mapTools"
      @tool-changed="onToolChanged"
      @reset-zoom="onResetZoom"
      @toggle-fullscreen="onToggleFullscreen"
      @capture-screen="onCaptureScreen"
      @clear-annotations="onClearAnnotations"
      @clear-measurement="onClearMeasurement"
    />
    
    <!-- 地图图例 - 右下角 -->
    <MapLegend 
      ref="mapLegend"
      @target-type-toggle="onTargetTypeToggle"
      @platform-type-toggle="onPlatformTypeToggle"
      @track-type-toggle="onTrackTypeToggle"
      @zone-type-toggle="onZoneTypeToggle"
      @show-all="onShowAll"
      @hide-all="onHideAll"
    />

    <!-- 全屏遮罩 -->
    <div v-if="isFullscreen" class="fullscreen-overlay" @click="exitFullscreen">
      <div class="fullscreen-content" @click.stop>
        <!-- 全屏模式下的地图内容 -->
        <div class="fullscreen-header">
          <h3 class="text-title">海图全屏显示</h3>
          <Button size="medium" square variant="danger" @click="exitFullscreen">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </Button>
        </div>
        <div class="fullscreen-map">
          <!-- 这里可以放置全屏地图内容 -->
          <div class="fullscreen-placeholder">
            <p class="text-subtitle">全屏地图视图</p>
            <p class="text-caption">按ESC键或点击关闭按钮退出全屏</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 状态提示 -->
    <div v-if="statusMessage" class="status-message" :class="statusType">
      {{ statusMessage }}
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, reactive } from 'vue'
import Button from '~/components/ui/Button.vue'
import MapTools from './MapTools.vue'
import MapMainArea from './MapMainArea.vue'
import MapLegend from './MapLegend.vue'

// 响应式数据
const activeTool = ref('select')
const visibleLayers = reactive({
  targets: true,
  platforms: true,
  tracks: true,
  zones: true,
  grid: false,
  terrain: true
})

// 组件引用
const mapMainArea = ref(null)
const mapTools = ref(null)
const mapLegend = ref(null)

// 状态数据
const isFullscreen = ref(false)
const statusMessage = ref('')
const statusType = ref('info')

// 当前坐标和视图信息
const currentCoordinates = ref({
  longitude: 120.5843,
  latitude: 31.2984
})
const currentZoom = ref(1.0)
const currentBearing = ref(0)

// 工具事件处理
const onToolChanged = (tool) => {
  activeTool.value = tool
  showStatus(`已切换到${getToolName(tool)}工具`, 'info')
}

const onResetZoom = () => {
  currentZoom.value = 1.0
  showStatus('已重置缩放级别', 'info')
}

const onToggleFullscreen = () => {
  isFullscreen.value = !isFullscreen.value
  if (isFullscreen.value) {
    showStatus('已进入全屏模式', 'info')
  }
}

const onCaptureScreen = () => {
  // 模拟截图功能
  showStatus('截图已保存', 'success')
}

const onClearAnnotations = () => {
  showStatus('已清除所有标注', 'warning')
}

const onClearMeasurement = () => {
  if (mapTools.value) {
    mapTools.value.setMeasurementResult(null)
  }
  showStatus('已清除测量结果', 'info')
}

// 图例事件处理
const onTargetTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}类型目标`, 'info')
}

const onPlatformTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}类型平台`, 'info')
}

const onTrackTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}类型航迹`, 'info')
}

const onZoneTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}控制区域`, 'info')
}

const onShowAll = () => {
  showStatus('已显示所有图例项目', 'success')
}

const onHideAll = () => {
  showStatus('已隐藏所有图例项目', 'warning')
}

// 坐标和视图更新
const updateCoordinates = (coords) => {
  currentCoordinates.value = coords
}

const updateZoom = (zoom) => {
  currentZoom.value = zoom
}

const updateBearing = (bearing) => {
  currentBearing.value = bearing
}

const updateMeasurement = (result) => {
  if (mapTools.value) {
    mapTools.value.setMeasurementResult(result)
  }
}

// 全屏相关
const exitFullscreen = () => {
  isFullscreen.value = false
  showStatus('已退出全屏模式', 'info')
}

// 键盘事件处理
const handleKeydown = (event) => {
  if (event.key === 'Escape' && isFullscreen.value) {
    exitFullscreen()
  }
}

// 状态消息显示
const showStatus = (message, type = 'info') => {
  statusMessage.value = message
  statusType.value = type
  
  // 3秒后自动清除消息
  setTimeout(() => {
    statusMessage.value = ''
  }, 3000)
}

// 工具名称映射
const getToolName = (tool) => {
  const toolNames = {
    'select': '选择',
    'zoom-in': '放大',
    'zoom-out': '缩小',
    'measure-distance': '测距',
    'measure-area': '测面积',
    'add-marker': '添加标记',
    'add-text': '文本标注',
    'draw-line': '绘制线条',
    'draw-area': '绘制区域'
  }
  return toolNames[tool] || tool
}

// 装备选择事件处理
const handleEquipmentSelected = (equipment) => {
  if (mapMainArea.value) {
    mapMainArea.value.setSelectedEquipment(equipment)
  }
}

// 生命周期钩子
onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
  
  // 模拟定期更新坐标和状态
  setInterval(() => {
    // 更新网络和数据状态
    const networkStatus = Math.random() > 0.1 ? 'online' : 'warning'
    const dataStatus = Math.random() > 0.05 ? 'online' : 'warning'
    
    if (mapMainArea.value) {
      mapMainArea.value.updateStatus(networkStatus, dataStatus)
    }
  }, 5000)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})

// 暴露方法供父组件使用
defineExpose({
  setActiveTool: (tool) => {
    activeTool.value = tool
    if (mapTools.value) {
      mapTools.value.setActiveTool(tool)
    }
  },
  updateCoordinates,
  updateZoom,
  updateBearing,
  exitFullscreen,
  clearSelection: () => {
    if (mapMainArea.value) {
      mapMainArea.value.clearSelection()
    }
  },
  updateVisibleLayers: (layers) => {
    Object.assign(visibleLayers, layers)
  }
})
</script>

<style scoped>
.map-container {
  flex: 1;
  position: relative;
  background: var(--primary-bg);
  border: 1px solid var(--border-color);
  overflow: hidden;
  box-shadow: var(--shadow-inset);
}

/* 确保地图组件层次清晰 */
.map-container > * {
  position: absolute;
}

/* 确保地图主区域完全填满容器 */
.map-container :deep(.map-area) {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}

/* 工具栏和图例的层级 */
.map-container :deep(.map-tools) {
  z-index: 30;
}

.map-container :deep(.map-legend) {
  z-index: 20;
}

/* 响应式布局调整 */
@media (max-width: 1024px) {
  .map-container :deep(.map-legend) {
    max-width: 160px;
    font-size: 10px;
  }
}

@media (max-width: 768px) {
  .map-container :deep(.map-tools) {
    left: var(--spacing-xs);
    top: var(--spacing-xs);
  }
  
  .map-container :deep(.map-legend) {
    right: var(--spacing-xs);
    bottom: var(--spacing-xs);
    max-width: 140px;
  }
}

/* 全屏样式 */
.fullscreen-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.95);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  backdrop-filter: blur(2px);
}

.fullscreen-content {
  width: 95vw;
  height: 95vh;
  background: var(--primary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  box-shadow: var(--shadow-lg);
}

.fullscreen-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md) var(--spacing-lg);
  border-bottom: 1px solid var(--border-color);
  background: var(--secondary-bg);
}

.close-btn {
  padding: var(--spacing-xs);
  min-width: auto;
}

.fullscreen-map {
  flex: 1;
  position: relative;
  background: var(--primary-bg);
}

.fullscreen-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: var(--spacing-md);
}

/* 状态消息样式 */
.status-message {
  position: absolute;
  top: var(--spacing-md);
  left: 50%;
  transform: translateX(-50%);
  padding: var(--spacing-sm) var(--spacing-md);
  border-radius: var(--radius-md);
  font-size: 12px;
  font-weight: 500;
  z-index: 100;
  max-width: 300px;
  text-align: center;
  box-shadow: var(--shadow-md);
  backdrop-filter: blur(4px);
  animation: slideDown 0.3s ease-out;
}

.status-message.info {
  background: var(--info-color);
  color: white;
}

.status-message.success {
  background: var(--success-color);
  color: white;
}

.status-message.warning {
  background: var(--warning-color);
  color: var(--primary-bg);
}

.status-message.error {
  background: var(--danger-color);
  color: white;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translate(-50%, -20px);
  }
  to {
    opacity: 1;
    transform: translate(-50%, 0);
  }
}
</style> 