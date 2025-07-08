<template>
  <div class="map-legend">
    <!-- 图例头部 -->
    <div class="legend-header" @click="toggleCollapse">
      <div class="legend-title">
        <Icon name="heroicons:rectangle-stack" size="12" />
        <span>图例</span>
      </div>
      <Button size="small" square variant="text">
        <Icon name="heroicons:chevron-down" size="10" :class="{ 'collapsed': isCollapsed }" />
      </Button>
    </div>

    <!-- 图例内容 -->
    <div v-if="!isCollapsed" class="legend-content">
      <!-- 快速切换 -->
      <div class="quick-controls">
        <Button size="small" square variant="ghost" title="全部显示" @click="showAll">
          <Icon name="heroicons:check-circle" size="10" />
        </Button>
        <Button size="small" square variant="ghost" title="全部隐藏" @click="hideAll">
          <Icon name="heroicons:x-circle" size="10" />
        </Button>
      </div>

      <!-- 目标类型 -->
      <div class="legend-group">
        <div class="group-header" @click="toggleGroup('targets')">
          <span class="group-title">目标</span>
          <span class="group-count">({{ getVisibleCount('targets') }}/{{ targetTypes.length }})</span>
          <Icon name="heroicons:chevron-down" size="8" :class="{ 'rotated': !expandedGroups.targets }" />
        </div>
        <div v-if="expandedGroups.targets" class="group-items">
          <div
            v-for="item in targetTypes" :key="item.id" 
            class="legend-item" @click="toggleItem('target', item.id)">
            <div
              class="item-symbol circle" 
              :style="{ backgroundColor: item.color, borderColor: item.strokeColor }"
              :class="{ 'disabled': !item.visible }"/>
            <span class="item-name" :class="{ 'disabled': !item.visible }">{{ item.name }}</span>
            <span class="item-count">{{ item.count }}</span>
          </div>
        </div>
      </div>

      <!-- 平台类型 -->
      <div class="legend-group">
        <div class="group-header" @click="toggleGroup('platforms')">
          <span class="group-title">平台</span>
          <span class="group-count">({{ getVisibleCount('platforms') }}/{{ platformTypes.length }})</span>
          <Icon name="heroicons:chevron-down" size="8" :class="{ 'rotated': !expandedGroups.platforms }" />
        </div>
        <div v-if="expandedGroups.platforms" class="group-items">
          <div
            v-for="item in platformTypes" :key="item.id" 
            class="legend-item" @click="toggleItem('platform', item.id)">
            <div
              class="item-symbol square" 
              :style="{ backgroundColor: item.color, borderColor: item.strokeColor }"
              :class="{ 'disabled': !item.visible }"/>
            <span class="item-name" :class="{ 'disabled': !item.visible }">{{ item.name }}</span>
            <span class="item-count">{{ item.count }}</span>
          </div>
        </div>
      </div>

      <!-- 航迹线 -->
      <div class="legend-group">
        <div class="group-header" @click="toggleGroup('tracks')">
          <span class="group-title">航迹</span>
          <span class="group-count">({{ getVisibleCount('tracks') }}/{{ trackTypes.length }})</span>
          <Icon name="heroicons:chevron-down" size="8" :class="{ 'rotated': !expandedGroups.tracks }" />
        </div>
        <div v-if="expandedGroups.tracks" class="group-items">
          <div
            v-for="item in trackTypes" :key="item.id" 
            class="legend-item" @click="toggleItem('track', item.id)">
            <div
              class="item-symbol line" 
              :style="{ borderColor: item.color, borderStyle: item.style }"
              :class="{ 'disabled': !item.visible }"/>
            <span class="item-name" :class="{ 'disabled': !item.visible }">{{ item.name }}</span>
          </div>
        </div>
      </div>

      <!-- 控制区域 -->
      <div class="legend-group">
        <div class="group-header" @click="toggleGroup('zones')">
          <span class="group-title">区域</span>
          <span class="group-count">({{ getVisibleCount('zones') }}/{{ zoneTypes.length }})</span>
          <Icon name="heroicons:chevron-down" size="8" :class="{ 'rotated': !expandedGroups.zones }" />
        </div>
        <div v-if="expandedGroups.zones" class="group-items">
          <div
            v-for="item in zoneTypes" :key="item.id" 
            class="legend-item" @click="toggleItem('zone', item.id)">
            <div
              class="item-symbol zone" 
              :style="{ borderColor: item.color }"
              :class="{ 'disabled': !item.visible }"/>
            <span class="item-name" :class="{ 'disabled': !item.visible }">{{ item.name }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import Button from '~/components/ui/Button.vue';

// 折叠状态
const isCollapsed = ref(false);
const expandedGroups = ref({
  targets: true,
  platforms: false,
  tracks: false,
  zones: false
});

// 数据 - 简化版本，移除基础图层
const targetTypes = ref([
  { id: 'friendly', name: '友军', color: '#00d9ff', strokeColor: '#0ea5e9', visible: true, count: 12 },
  { id: 'commercial', name: '商船', color: '#4ade80', strokeColor: '#22c55e', visible: true, count: 25 },
  { id: 'unknown', name: '未知', color: '#f59e0b', strokeColor: '#d97706', visible: true, count: 8 },
  { id: 'suspicious', name: '可疑', color: '#ef4444', strokeColor: '#dc2626', visible: true, count: 3 }
]);

const platformTypes = ref([
  { id: 'submarine', name: '潜艇', color: '#1e40af', strokeColor: '#1d4ed8', visible: true, count: 4 },
  { id: 'destroyer', name: '驱逐舰', color: '#7c3aed', strokeColor: '#6d28d9', visible: true, count: 6 },
  { id: 'frigate', name: '护卫舰', color: '#0891b2', strokeColor: '#0e7490', visible: true, count: 8 },
  { id: 'aircraft', name: '飞机', color: '#e11d48', strokeColor: '#be185d', visible: false, count: 2 }
]);

const trackTypes = ref([
  { id: 'primary', name: '主航迹', color: '#60a5fa', style: 'solid', visible: true },
  { id: 'secondary', name: '副航迹', color: '#34d399', style: 'dashed', visible: true },
  { id: 'predicted', name: '预测航迹', color: '#fbbf24', style: 'dotted', visible: false }
]);

const zoneTypes = ref([
  { id: 'patrol', name: '巡逻区', color: '#fbbf24', visible: true },
  { id: 'restricted', name: '禁航区', color: '#ef4444', visible: true },
  { id: 'safe', name: '安全区', color: '#22c55e', visible: false }
]);

// 事件
const emit = defineEmits([
  'target-type-toggle',
  'platform-type-toggle', 
  'track-type-toggle',
  'zone-type-toggle',
  'show-all',
  'hide-all'
]);

// 方法
const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value;
};

const toggleGroup = (groupName) => {
  expandedGroups.value[groupName] = !expandedGroups.value[groupName];
};

const toggleItem = (type, id) => {
  let items;
  switch (type) {
  case 'target':
    items = targetTypes.value;
    break;
  case 'platform':
    items = platformTypes.value;
    break;
  case 'track':
    items = trackTypes.value;
    break;
  case 'zone':
    items = zoneTypes.value;
    break;
  }
  
  const item = items.find(item => item.id === id);
  if (item) {
    item.visible = !item.visible;
    emit(`${type}-type-toggle`, { id, visible: item.visible });
  }
};

const getVisibleCount = (type) => {
  switch (type) {
  case 'targets':
    return targetTypes.value.filter(item => item.visible).length;
  case 'platforms':
    return platformTypes.value.filter(item => item.visible).length;
  case 'tracks':
    return trackTypes.value.filter(item => item.visible).length;
  case 'zones':
    return zoneTypes.value.filter(item => item.visible).length;
  default:
    return 0;
  }
};

const showAll = () => {
  targetTypes.value.forEach(item => { item.visible = true; });
  platformTypes.value.forEach(item => { item.visible = true; });
  trackTypes.value.forEach(item => { item.visible = true; });
  zoneTypes.value.forEach(item => { item.visible = true; });
  emit('show-all');
};

const hideAll = () => {
  targetTypes.value.forEach(item => { item.visible = false; });
  platformTypes.value.forEach(item => { item.visible = false; });
  trackTypes.value.forEach(item => { item.visible = false; });
  zoneTypes.value.forEach(item => { item.visible = false; });
  emit('hide-all');
};
</script>

<style scoped>
.map-legend {
  position: absolute;
  bottom: var(--spacing-md);
  right: var(--spacing-md);
  background: var(--panel-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  backdrop-filter: blur(4px);
  box-shadow: var(--shadow-md);
  min-width: 160px;
  max-width: 200px;
  z-index: 20;
  overflow: hidden;
}

.legend-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.legend-header:hover {
  background: linear-gradient(145deg, var(--border-color-light), var(--tertiary-bg));
}

.legend-title {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: 11px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.collapse-btn {
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.collapse-btn:hover {
  color: var(--text-primary);
  background: var(--border-color);
}

.collapse-btn.collapsed svg {
  transform: rotate(-90deg);
}

.legend-content {
  padding: var(--spacing-xs);
  max-height: 300px;
  overflow-y: auto;
}

.quick-controls {
  display: flex;
  gap: 2px;
  margin-bottom: var(--spacing-xs);
  padding-bottom: var(--spacing-xs);
  border-bottom: 1px solid var(--divider-color);
}

.quick-btn {
  flex: 1;
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

.quick-btn:hover {
  border-color: var(--border-color-light);
  color: var(--text-primary);
}

.show-all:hover {
  border-color: var(--success-color);
  color: var(--success-color);
}

.hide-all:hover {
  border-color: var(--danger-color);
  color: var(--danger-color);
}

.legend-items {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.legend-group {
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.group-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 2px 4px;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.group-header:hover {
  background: var(--tertiary-bg);
  border-color: var(--border-color-light);
}

.group-title {
  font-size: 10px;
  font-weight: 600;
  color: var(--text-accent);
  text-transform: uppercase;
  letter-spacing: 0.2px;
}

.group-count {
  font-size: 9px;
  color: var(--text-muted);
  margin-left: auto;
}

.group-header svg {
  stroke-width: 2;
  color: var(--text-muted);
  transition: transform var(--transition-fast);
}

.group-header svg.rotated {
  transform: rotate(-90deg);
}

.group-items {
  margin-top: 2px;
  padding-left: var(--spacing-xs);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: 1px 2px;
  cursor: pointer;
  transition: all var(--transition-fast);
  border-radius: var(--radius-sm);
}

.legend-item:hover {
  background: var(--border-color);
}

.item-symbol {
  flex-shrink: 0;
  border: 1px solid;
  transition: all var(--transition-fast);
}

.item-symbol.circle {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  box-shadow: 0 0 2px currentColor;
}

.item-symbol.square {
  width: 8px;
  height: 8px;
  border-radius: 1px;
}

.item-symbol.line {
  width: 12px;
  height: 1px;
  border-top: 2px solid;
  border-left: none;
  border-right: none;
  border-bottom: none;
}

.item-symbol.zone {
  width: 10px;
  height: 6px;
  border: 1px dashed;
  background: transparent;
}

.item-symbol.disabled {
  opacity: 0.3;
}

.item-name {
  font-size: 10px;
  color: var(--text-secondary);
  transition: all var(--transition-fast);
}

.item-name.disabled {
  opacity: 0.5;
  text-decoration: line-through;
}

.item-count {
  margin-left: auto;
  font-size: 9px;
  color: var(--text-muted);
  font-family: var(--font-family-mono);
}
</style> 