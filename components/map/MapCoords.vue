<template>
  <div class="map-coords">
    <!-- 选中装备信息 -->
    <div v-if="selectedEquipment" class="equipment-info">
      <div class="equipment-header">
        <div class="equipment-name">{{ selectedEquipment.name }}</div>
        <div class="equipment-type">{{ selectedEquipment.type }}</div>
      </div>
      
      <div class="equipment-coords">
        <div class="coord-row">
          <span class="coord-label">经度</span>
          <span class="coord-value text-monospace">{{ formatCoordinate(selectedEquipment.longitude, 'longitude') }}</span>
        </div>
        <div class="coord-row">
          <span class="coord-label">纬度</span>
          <span class="coord-value text-monospace">{{ formatCoordinate(selectedEquipment.latitude, 'latitude') }}</span>
        </div>
        <div class="coord-row">
          <span class="coord-label">航向</span>
          <span class="coord-value text-monospace">{{ selectedEquipment.heading }}°</span>
        </div>
        <div class="coord-row">
          <span class="coord-label">航速</span>
          <span class="coord-value text-monospace">{{ selectedEquipment.speed }} kn</span>
        </div>
      </div>

      <div class="equipment-status">
        <div class="status-item">
          <div class="status-indicator" :class="`status-${selectedEquipment.status}`"/>
          <span class="status-text">{{ getStatusText(selectedEquipment.status) }}</span>
        </div>
        <div class="last-update">
          {{ formatTime(selectedEquipment.lastUpdate) }}
        </div>
      </div>
    </div>

    <!-- 无选中时的提示 -->
    <div v-else class="no-selection">
      <div class="no-selection-icon">
        <Icon name="heroicons:map-pin" size="16" />
      </div>
      <div class="no-selection-text">未选中装备</div>
      <div class="no-selection-hint">点击地图上的装备查看详情</div>
    </div>

    <!-- 坐标格式切换按钮 -->
    <Button v-if="selectedEquipment" size="small" square variant="text" title="切换坐标格式" @click="switchCoordinateFormat">
      <Icon name="heroicons:arrows-right-left" size="10" />
    </Button>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import Button from '~/components/ui/Button.vue';

// 坐标格式
const coordinateFormats = ref([
  { id: 'dd', name: '度(DD)', precision: 6 },
  { id: 'dms', name: '度分秒(DMS)', precision: 2 },
  { id: 'ddm', name: '度分(DDM)', precision: 3 }
]);

const currentFormatIndex = ref(0);

// 模拟选中的装备数据
const selectedEquipment = ref({
  id: 'ship_001',
  name: '驱逐舰-001',
  type: '驱逐舰',
  longitude: 120.584312,
  latitude: 31.298456,
  heading: 275,
  speed: 12.5,
  status: 'online', // online, warning, offline
  lastUpdate: new Date()
});

// 当前坐标格式
const currentFormat = computed(() => coordinateFormats.value[currentFormatIndex.value]);

// 格式化坐标
const formatCoordinate = (value, type) => {
  const format = currentFormat.value;
  const absValue = Math.abs(value);
  const direction = type === 'longitude' 
    ? (value >= 0 ? 'E' : 'W') 
    : (value >= 0 ? 'N' : 'S');

  switch (format.id) {
  case 'dd':
    return `${absValue.toFixed(format.precision)}° ${direction}`;
    
  case 'dms': {
    const degrees = Math.floor(absValue);
    const minutes = Math.floor((absValue - degrees) * 60);
    const seconds = ((absValue - degrees) * 60 - minutes) * 60;
    return `${degrees}°${minutes}'${seconds.toFixed(format.precision)}" ${direction}`;
  }
    
  case 'ddm': {
    const deg = Math.floor(absValue);
    const min = (absValue - deg) * 60;
    return `${deg}°${min.toFixed(format.precision)}' ${direction}`;
  }
    
  default:
    return `${absValue.toFixed(6)}° ${direction}`;
  }
};

// 格式化时间
const formatTime = (date) => {
  const now = new Date();
  const diff = Math.floor((now - date) / 1000);
  
  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  return date.toLocaleDateString();
};

// 获取状态文本
const getStatusText = (status) => {
  switch (status) {
  case 'online': return '在线';
  case 'warning': return '警告';
  case 'offline': return '离线';
  default: return '未知';
  }
};

// 切换坐标格式
const switchCoordinateFormat = () => {
  currentFormatIndex.value = (currentFormatIndex.value + 1) % coordinateFormats.value.length;
};

// 设置选中装备（供父组件调用）
const setSelectedEquipment = (equipment) => {
  selectedEquipment.value = equipment;
};

// 清除选中
const clearSelection = () => {
  selectedEquipment.value = null;
};

// 定义事件
const _emit = defineEmits(['equipment-selected', 'format-changed']);

// 暴露方法给父组件
defineExpose({
  setSelectedEquipment,
  clearSelection
});
</script>

<style scoped>
.map-coords {
  position: absolute;
  top: var(--spacing-sm);
  right: var(--spacing-sm);
  background: var(--panel-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  backdrop-filter: blur(4px);
  box-shadow: var(--shadow-md);
  width: 200px; /* 与图例宽度保持一致 */
  z-index: 25;
  overflow: hidden;
}

.equipment-info {
  padding: var(--spacing-sm);
}

.equipment-header {
  margin-bottom: var(--spacing-sm);
  padding-bottom: var(--spacing-xs);
  border-bottom: 1px solid var(--divider-color);
}

.equipment-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.equipment-type {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.equipment-coords {
  margin-bottom: var(--spacing-sm);
}

.coord-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.coord-row:last-child {
  margin-bottom: 0;
}

.coord-label {
  font-size: 11px;
  color: var(--text-secondary);
}

.coord-value {
  font-size: 12px;
  color: var(--text-accent);
  font-weight: 600;
}

.equipment-status {
  padding-top: var(--spacing-xs);
  border-top: 1px solid var(--divider-color);
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  margin-bottom: 4px;
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  position: relative;
}

.status-indicator::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: currentColor;
  box-shadow: 0 0 4px currentColor;
}

.status-online {
  color: var(--success-color);
}

.status-warning {
  color: var(--warning-color);
}

.status-offline {
  color: var(--danger-color);
}

.status-text {
  font-size: 11px;
  color: var(--text-secondary);
}

.last-update {
  font-size: 10px;
  color: var(--text-muted);
  text-align: right;
}

.no-selection {
  padding: var(--spacing-md);
  text-align: center;
  color: var(--text-muted);
}

.no-selection-icon {
  margin-bottom: var(--spacing-xs);
  opacity: 0.6;
}

.no-selection-text {
  font-size: 12px;
  font-weight: 600;
  margin-bottom: 4px;
}

.no-selection-hint {
  font-size: 10px;
  opacity: 0.8;
}

.format-btn {
  position: absolute;
  top: var(--spacing-xs);
  right: var(--spacing-xs);
  width: 20px;
  height: 20px;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.format-btn:hover {
  border-color: var(--border-color-light);
  color: var(--text-primary);
  background: var(--tertiary-bg);
}

.format-btn svg {
  stroke-width: 2;
}

/* 添加复选框样式以保持与图例一致 */
.layer-checkbox {
  accent-color: var(--info-color);
  width: 12px;
  height: 12px;
}

.layer-label {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  cursor: pointer;
  flex: 1;
  font-size: 10px;
}

.layer-icon {
  font-size: 6px;
  filter: drop-shadow(0 0 2px currentColor);
}
</style> 