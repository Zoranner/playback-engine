<template>
  <div class="relative flex-1 overflow-hidden border border-border bg-red-200 shadow-inner">
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
import MapMainArea from './MapMainArea.vue';

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

// 状态数据
const statusMessage = ref('');
const statusType = ref('info');

// 当前坐标和视图信息
const currentCoordinates = ref({
  longitude: 120.5843,
  latitude: 31.2984,
});
const currentZoom = ref(1.0);
const currentBearing = ref(0);

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
  // 这里可以根据需要处理测量结果
};

// 状态消息显示
const showStatus = (message, type = 'info') => {
  statusMessage.value = message;
  statusType.value = type;
  setTimeout(() => {
    statusMessage.value = '';
  }, 3000);
};

// 装备选择事件处理
const handleEquipmentSelected = equipment => {
  if (mapMainArea.value) {
    mapMainArea.value.setSelectedEquipment(equipment);
  }
};

// 生命周期钩子
onMounted(() => {
  // 这里可以添加需要的初始化逻辑
});

onUnmounted(() => {
  // 这里可以添加需要的清理逻辑
});

// 暴露方法供父组件使用
defineExpose({
  setActiveTool: tool => {
    activeTool.value = tool;
  },
  updateCoordinates,
  updateZoom,
  updateBearing,
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
