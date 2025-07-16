<template>
  <GroupBox
    title="坐标信息"
    class="min-h-40 flex-none"
  >
    <!-- 选中装备信息 -->
    <div
      v-if="selectedEquipment"
      class="equipment-info"
    >
      <div class="mb-sm border-b border-border-divider pb-xs">
        <div class="mb-0.5 font-semibold text-text-primary">{{ selectedEquipment.name }}</div>
        <div class="text-text-secondary">{{ selectedEquipment.type }}</div>
      </div>

      <div class="mb-sm">
        <div class="min-h-4.5 mb-xs flex items-center justify-between">
          <span class="w-9 flex-none font-medium text-text-secondary">经度</span>
          <span
            class="flex-1 cursor-pointer text-right text-text-accent"
            @click="switchCoordinateFormat"
            >{{ formatCoordinate(selectedEquipment.longitude, 'longitude') }}</span
          >
        </div>
        <div class="min-h-4.5 mb-xs flex items-center justify-between">
          <span class="w-9 flex-none font-medium text-text-secondary">纬度</span>
          <span
            class="flex-1 cursor-pointer text-right text-text-accent"
            @click="switchCoordinateFormat"
            >{{ formatCoordinate(selectedEquipment.latitude, 'latitude') }}</span
          >
        </div>
        <div class="min-h-4.5 mb-xs flex items-center justify-between">
          <span class="w-9 flex-none font-medium text-text-secondary">航向</span>
          <span class="flex-1 text-right text-text-accent">{{ selectedEquipment.heading }}°</span>
        </div>
        <div class="min-h-4.5 mb-xs flex items-center justify-between">
          <span class="w-9 flex-none font-medium text-text-secondary">航速</span>
          <span class="flex-1 text-right text-text-accent">{{ selectedEquipment.speed }} kn</span>
        </div>
      </div>
    </div>

    <!-- 无选中时的提示 -->
    <div
      v-else
      class="py-md text-center text-text-muted"
    >
      <div class="mb-sm opacity-60">
        <Icon
          name="heroicons:map-pin"
          size="16"
        />
      </div>
      <div class="mb-xs text-text-secondary">未选中装备</div>
      <div class="text-text-muted opacity-70">点击地图上的装备查看详情</div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref, computed } from 'vue';
import GroupBox from '~/components/display/GroupBox.vue';

// 坐标格式
const coordinateFormats = ref([
  { id: 'dd', name: '度(DD)', precision: 6 },
  { id: 'dms', name: '度分秒(DMS)', precision: 2 },
  { id: 'ddm', name: '度分(DDM)', precision: 3 },
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
  lastUpdate: new Date(),
});

// 当前坐标格式
const currentFormat = computed(() => coordinateFormats.value[currentFormatIndex.value]);

// 格式化坐标
const formatCoordinate = (value, type) => {
  const format = currentFormat.value;
  const absValue = Math.abs(value);
  const direction = type === 'longitude' ? (value >= 0 ? 'E' : 'W') : value >= 0 ? 'N' : 'S';

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
const formatTime = date => {
  const now = new Date();
  const diff = Math.floor((now - date) / 1000);

  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  return date.toLocaleDateString();
};

// 切换坐标格式
const switchCoordinateFormat = () => {
  currentFormatIndex.value = (currentFormatIndex.value + 1) % coordinateFormats.value.length;
};

// 设置选中装备（供父组件调用）
const setSelectedEquipment = equipment => {
  selectedEquipment.value = equipment;
};

// 清除选中
const clearSelection = () => {
  selectedEquipment.value = null;
};

// 定义事件
defineEmits(['equipment-selected', 'format-changed']);

// 暴露方法给父组件
defineExpose({
  setSelectedEquipment,
  clearSelection,
});
</script>

<!-- 完全移除 <style> 标签 -->
