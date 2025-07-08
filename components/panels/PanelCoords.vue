<template>
  <MilitaryGroupBox title="坐标信息" class="coords-container">
    <!-- 选中装备信息 -->
    <div v-if="selectedEquipment" class="equipment-info">
      <div class="equipment-header">
        <div class="equipment-name">{{ selectedEquipment.name }}</div>
        <div class="equipment-type">{{ selectedEquipment.type }}</div>
      </div>
      
      <div class="equipment-coords">
        <div class="coord-row">
          <span class="coord-label">经度</span>
          <span class="coord-value text-monospace" @click="switchCoordinateFormat">{{ formatCoordinate(selectedEquipment.longitude, 'longitude') }}</span>
        </div>
        <div class="coord-row">
          <span class="coord-label">纬度</span>
          <span class="coord-value text-monospace" @click="switchCoordinateFormat">{{ formatCoordinate(selectedEquipment.latitude, 'latitude') }}</span>
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
    </div>

    <!-- 无选中时的提示 -->
    <div v-else class="no-selection">
      <div class="no-selection-icon">
        <Icon name="heroicons:map-pin" size="16" />
      </div>
      <div class="no-selection-text">未选中装备</div>
      <div class="no-selection-hint">点击地图上的装备查看详情</div>
    </div>
  </MilitaryGroupBox>
</template>

<script setup>
import { ref, computed } from 'vue'
import MilitaryGroupBox from '~/components/ui/MilitaryGroupBox.vue'

// 坐标格式
const coordinateFormats = ref([
  { id: 'dd', name: '度(DD)', precision: 6 },
  { id: 'dms', name: '度分秒(DMS)', precision: 2 },
  { id: 'ddm', name: '度分(DDM)', precision: 3 }
])

const currentFormatIndex = ref(0)

// 模拟选中的装备数据
const selectedEquipment = ref({
  id: 'ship_001',
  name: '驱逐舰-001',
  type: '驱逐舰',
  longitude: 120.584312,
  latitude: 31.298456,
  heading: 275,
  speed: 12.5,
  lastUpdate: new Date()
})

// 当前坐标格式
const currentFormat = computed(() => coordinateFormats.value[currentFormatIndex.value])

// 格式化坐标
const formatCoordinate = (value, type) => {
  const format = currentFormat.value
  const absValue = Math.abs(value)
  const direction = type === 'longitude' 
    ? (value >= 0 ? 'E' : 'W') 
    : (value >= 0 ? 'N' : 'S')

  switch (format.id) {
    case 'dd':
      return `${absValue.toFixed(format.precision)}° ${direction}`
    
    case 'dms': {
      const degrees = Math.floor(absValue)
      const minutes = Math.floor((absValue - degrees) * 60)
      const seconds = ((absValue - degrees) * 60 - minutes) * 60
      return `${degrees}°${minutes}'${seconds.toFixed(format.precision)}" ${direction}`
    }
    
    case 'ddm': {
      const deg = Math.floor(absValue)
      const min = (absValue - deg) * 60
      return `${deg}°${min.toFixed(format.precision)}' ${direction}`
    }
    
    default:
      return `${absValue.toFixed(6)}° ${direction}`
  }
}

// 格式化时间
const formatTime = (date) => {
  const now = new Date()
  const diff = Math.floor((now - date) / 1000)
  
  if (diff < 60) return `${diff}秒前`
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`
  return date.toLocaleDateString()
}

// 切换坐标格式
const switchCoordinateFormat = () => {
  currentFormatIndex.value = (currentFormatIndex.value + 1) % coordinateFormats.value.length
}

// 设置选中装备（供父组件调用）
const setSelectedEquipment = (equipment) => {
  selectedEquipment.value = equipment
}

// 清除选中
const clearSelection = () => {
  selectedEquipment.value = null
}

// 定义事件
defineEmits(['equipment-selected', 'format-changed'])

// 暴露方法给父组件
defineExpose({
  setSelectedEquipment,
  clearSelection
})
</script>

<style scoped>
.coords-container {
  flex: 0 0 auto;
  min-height: 160px;
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
}

.equipment-coords {
  margin-bottom: var(--spacing-sm);
}

.coord-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-xs);
  min-height: 18px;
}

.coord-label {
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 500;
  flex: 0 0 35px;
}

.coord-value {
  font-size: 12px;
  color: var(--text-accent);
  font-weight: 600;
  text-align: right;
  flex: 1;
  cursor: pointer;
  transition: all var(--transition-fast);
  padding: 2px 4px;
  border-radius: var(--radius-xs);
}

.coord-value:hover {
  background: var(--border-color);
  color: var(--text-primary);
}

.no-selection {
  text-align: center;
  color: var(--text-tertiary);
  padding: var(--spacing-md) 0;
}

.no-selection-icon {
  margin-bottom: var(--spacing-sm);
  opacity: 0.6;
}

.no-selection-text {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
}

.no-selection-hint {
  font-size: 10px;
  color: var(--text-tertiary);
  opacity: 0.8;
}
</style> 