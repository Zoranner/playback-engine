<template>
  <div
    class="relative flex-1 overflow-hidden border border-border bg-background-primary shadow-inner"
  >
    <!-- 主要地图区域 -->
    <MapMainArea
      ref="mapMainArea"
      class="absolute inset-0"
      :active-tool="activeTool"
      :visible-layers="visibleLayers"
      @coordinates-changed="updateCoordinates"
      @zoom-changed="updateZoom"
      @bearing-changed="updateBearing"
      @measurement-result="updateMeasurement"
      @equipment-selected="handleEquipmentSelected"
    />

    <!-- 地图工具 - 响应式定位 -->
    <MapTools
      ref="mapTools"
      class="absolute left-md top-md z-30 md:left-xs md:top-xs"
      @tool-changed="onToolChanged"
      @reset-zoom="onResetZoom"
      @toggle-fullscreen="onToggleFullscreen"
      @capture-screen="onCaptureScreen"
      @clear-annotations="onClearAnnotations"
      @clear-measurement="onClearMeasurement"
    />

    <!-- 地图图例 - 响应式定位和大小 -->
    <MapLegend
      ref="mapLegend"
      class="absolute bottom-md right-md z-20 max-w-[180px] md:bottom-xs md:right-xs md:max-w-[140px] lg:max-w-[160px]"
      @target-type-toggle="onTargetTypeToggle"
      @platform-type-toggle="onPlatformTypeToggle"
      @track-type-toggle="onTrackTypeToggle"
      @zone-type-toggle="onZoneTypeToggle"
      @show-all="onShowAll"
      @hide-all="onHideAll"
    />

    <!-- 全屏遮罩 -->
    <div
      v-if="isFullscreen"
      class="fixed inset-0 z-modal flex items-center justify-center bg-background-overlay"
      @click="exitFullscreen"
    >
      <div
        class="flex h-[95vh] w-[95vw] flex-col overflow-hidden rounded-lg border border-border bg-background-primary shadow-lg"
        @click.stop
      >
        <!-- 全屏模式下的地图内容 -->
        <div
          class="flex items-center justify-between border-b border-border bg-background-secondary px-lg py-md"
        >
          <h3 class="text-title">海图全屏显示</h3>
          <Button size="medium" square variant="danger" @click="exitFullscreen">
            <Icon name="heroicons:x-mark" size="16" />
          </Button>
        </div>
        <div class="relative flex-1 bg-background-primary">
          <!-- 这里可以放置全屏地图内容 -->
          <div class="flex h-full flex-col items-center justify-center gap-md">
            <p class="text-subtitle">全屏地图视图</p>
            <p class="text-caption">按ESC键或点击关闭按钮退出全屏</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 状态提示（简化版，移除动画） -->
    <div
      v-if="statusMessage"
      class="absolute left-1/2 top-md z-[100] max-w-[300px] -translate-x-1/2 transform rounded-md px-md py-sm text-center font-medium text-white shadow-md"
      :class="{
        'bg-info': statusType === 'info',
        'bg-success': statusType === 'success',
        'bg-warning text-background-primary': statusType === 'warning',
        'bg-danger': statusType === 'error',
      }"
    >
      {{ statusMessage }}
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, reactive } from 'vue';
import Button from '~/components/base/Button.vue';
import MapTools from './MapTools.vue';
import MapMainArea from './MapMainArea.vue';
import MapLegend from './MapLegend.vue';

// 响应式数据
const activeTool = ref('select');
const visibleLayers = reactive({
  targets: true,
  platforms: true,
  tracks: true,
  zones: true,
  grid: false,
  terrain: true,
});

// 组件引用
const mapMainArea = ref(null);
const mapTools = ref(null);
const mapLegend = ref(null);

// 状态数据
const isFullscreen = ref(false);
const statusMessage = ref('');
const statusType = ref('info');

// 当前坐标和视图信息
const currentCoordinates = ref({
  longitude: 120.5843,
  latitude: 31.2984,
});
const currentZoom = ref(1.0);
const currentBearing = ref(0);

// 工具事件处理
const onToolChanged = tool => {
  activeTool.value = tool;
  showStatus(`已切换到${getToolName(tool)}工具`, 'info');
};

const onResetZoom = () => {
  currentZoom.value = 1.0;
  showStatus('已重置缩放级别', 'info');
};

const onToggleFullscreen = () => {
  isFullscreen.value = !isFullscreen.value;
  if (isFullscreen.value) {
    showStatus('已进入全屏模式', 'info');
  }
};

const onCaptureScreen = () => {
  // 模拟截图功能
  showStatus('截图已保存', 'success');
};

const onClearAnnotations = () => {
  showStatus('已清除所有标注', 'warning');
};

const onClearMeasurement = () => {
  if (mapTools.value) {
    mapTools.value.setMeasurementResult(null);
  }
  showStatus('已清除测量结果', 'info');
};

// 图例事件处理
const onTargetTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}类型目标`, 'info');
};

const onPlatformTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}类型平台`, 'info');
};

const onTrackTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}类型航迹`, 'info');
};

const onZoneTypeToggle = ({ id, visible }) => {
  showStatus(`${visible ? '显示' : '隐藏'}了${id}控制区域`, 'info');
};

const onShowAll = () => {
  showStatus('已显示所有图例项目', 'success');
};

const onHideAll = () => {
  showStatus('已隐藏所有图例项目', 'warning');
};

// 坐标和视图更新
const updateCoordinates = coords => {
  currentCoordinates.value = coords;
};

const updateZoom = zoom => {
  currentZoom.value = zoom;
};

const updateBearing = bearing => {
  currentBearing.value = bearing;
};

const updateMeasurement = result => {
  if (mapTools.value) {
    mapTools.value.setMeasurementResult(result);
  }
};

// 全屏相关
const exitFullscreen = () => {
  isFullscreen.value = false;
  showStatus('已退出全屏模式', 'info');
};

// 键盘事件处理
const handleKeydown = event => {
  if (event.key === 'Escape' && isFullscreen.value) {
    exitFullscreen();
  }
};

// 状态消息显示
const showStatus = (message, type = 'info') => {
  statusMessage.value = message;
  statusType.value = type;

  // 3秒后自动清除消息
  setTimeout(() => {
    statusMessage.value = '';
  }, 3000);
};

// 工具名称映射
const getToolName = tool => {
  const toolNames = {
    select: '选择',
    'zoom-in': '放大',
    'zoom-out': '缩小',
    'measure-distance': '测距',
    'measure-area': '测面积',
    'add-marker': '添加标记',
    'add-text': '文本标注',
    'draw-line': '绘制线条',
    'draw-area': '绘制区域',
  };
  return toolNames[tool] || tool;
};

// 装备选择事件处理
const handleEquipmentSelected = equipment => {
  if (mapMainArea.value) {
    mapMainArea.value.setSelectedEquipment(equipment);
  }
};

// 生命周期钩子
onMounted(() => {
  document.addEventListener('keydown', handleKeydown);

  // 模拟定期更新坐标和状态
  setInterval(() => {
    // 更新网络和数据状态
    const networkStatus = Math.random() > 0.1 ? 'online' : 'warning';
    const dataStatus = Math.random() > 0.05 ? 'online' : 'warning';

    if (mapMainArea.value) {
      mapMainArea.value.updateStatus(networkStatus, dataStatus);
    }
  }, 5000);
});

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown);
});

// 暴露方法供父组件使用
defineExpose({
  setActiveTool: tool => {
    activeTool.value = tool;
    if (mapTools.value) {
      mapTools.value.setActiveTool(tool);
    }
  },
  updateCoordinates,
  updateZoom,
  updateBearing,
  exitFullscreen,
  clearSelection: () => {
    if (mapMainArea.value) {
      mapMainArea.value.clearSelection();
    }
  },
  updateVisibleLayers: layers => {
    Object.assign(visibleLayers, layers);
  },
});
</script>
