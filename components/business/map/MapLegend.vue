<template>
  <div
    class="absolute bottom-md right-md z-20 min-w-48 max-w-64 overflow-hidden rounded-md border border-border bg-background-panel shadow-lg backdrop-blur-sm transition-all duration-normal hover:shadow-xl"
  >
    <!-- 图例标题 -->
    <div
      class="flex cursor-pointer items-center justify-between bg-gradient-to-r from-background-secondary to-background-tertiary px-md py-sm transition-all duration-fast hover:from-background-tertiary hover:to-background-secondary hover:shadow-glow-subtle"
      @click="toggleCollapse"
    >
      <div class="flex items-center gap-sm font-semibold tracking-wide text-text-primary">
        <Icon name="heroicons:rectangle-stack" size="14" class="text-primary" />
        <span>图例</span>
      </div>
      <Icon name="heroicons:chevron-down" size="12" :class="collapseIconClasses" />
    </div>

    <!-- 图例内容 -->
    <div
      v-if="!isCollapsed"
      class="max-h-80 overflow-y-auto bg-gradient-to-br from-background-panel to-background-secondary"
    >
      <Collapse
        :items="collapseItems"
        :default-active-key="['targets']"
        size="small"
        class="map-legend-collapse"
      >
        <!-- 目标类型内容 -->
        <template #content-targets>
          <div class="flex flex-col gap-1">
            <div
              v-for="item in targetTypes"
              :key="item.id"
              class="group flex cursor-pointer items-center gap-sm rounded-sm px-sm py-xs transition-all duration-fast hover:bg-background-tertiary/50"
              @click="toggleItem('target', item.id)"
            >
              <div
                :class="getSymbolClasses('circle', item.visible)"
                :style="{
                  backgroundColor: item.color,
                  borderColor: item.strokeColor,
                }"
              />
              <span :class="getNameClasses(item.visible)">{{ item.name }}</span>
              <span
                class="ml-auto font-mono text-xs text-text-muted group-hover:text-text-secondary"
                >{{ item.count }}</span
              >
            </div>
          </div>
        </template>

        <!-- 平台类型内容 -->
        <template #content-platforms>
          <div class="flex flex-col gap-1">
            <div
              v-for="item in platformTypes"
              :key="item.id"
              class="group flex cursor-pointer items-center gap-sm rounded-sm px-sm py-xs transition-all duration-fast hover:bg-background-tertiary/50"
              @click="toggleItem('platform', item.id)"
            >
              <div
                :class="getSymbolClasses('square', item.visible)"
                :style="{
                  backgroundColor: item.color,
                  borderColor: item.strokeColor,
                }"
              />
              <span :class="getNameClasses(item.visible)">{{ item.name }}</span>
              <span
                class="ml-auto font-mono text-xs text-text-muted group-hover:text-text-secondary"
                >{{ item.count }}</span
              >
            </div>
          </div>
        </template>

        <!-- 航迹线内容 -->
        <template #content-tracks>
          <div class="flex flex-col gap-1">
            <div
              v-for="item in trackTypes"
              :key="item.id"
              class="group flex cursor-pointer items-center gap-sm rounded-sm px-sm py-xs transition-all duration-fast hover:bg-background-tertiary/50"
              @click="toggleItem('track', item.id)"
            >
              <div
                :class="getSymbolClasses('line', item.visible)"
                :style="{ borderColor: item.color, borderStyle: item.style }"
              />
              <span :class="getNameClasses(item.visible)">{{ item.name }}</span>
            </div>
          </div>
        </template>

        <!-- 控制区域内容 -->
        <template #content-zones>
          <div class="flex flex-col gap-1">
            <div
              v-for="item in zoneTypes"
              :key="item.id"
              class="group flex cursor-pointer items-center gap-sm rounded-sm px-sm py-xs transition-all duration-fast hover:bg-background-tertiary/50"
              @click="toggleItem('zone', item.id)"
            >
              <div
                :class="getSymbolClasses('zone', item.visible)"
                :style="{ borderColor: item.color }"
              />
              <span :class="getNameClasses(item.visible)">{{ item.name }}</span>
            </div>
          </div>
        </template>
      </Collapse>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import Collapse from '~/components/layout/Collapse.vue';

// 折叠状态
const isCollapsed = ref(false);

// 数据 - 简化版本，移除基础图层
const targetTypes = ref([
  {
    id: 'friendly',
    name: '友军',
    color: '#00d9ff',
    strokeColor: '#0ea5e9',
    visible: true,
    count: 12,
  },
  {
    id: 'commercial',
    name: '商船',
    color: '#4ade80',
    strokeColor: '#22c55e',
    visible: true,
    count: 25,
  },
  {
    id: 'unknown',
    name: '未知',
    color: '#f59e0b',
    strokeColor: '#d97706',
    visible: true,
    count: 8,
  },
  {
    id: 'suspicious',
    name: '可疑',
    color: '#ef4444',
    strokeColor: '#dc2626',
    visible: true,
    count: 3,
  },
]);

const platformTypes = ref([
  {
    id: 'submarine',
    name: '潜艇',
    color: '#1e40af',
    strokeColor: '#1d4ed8',
    visible: true,
    count: 4,
  },
  {
    id: 'destroyer',
    name: '驱逐舰',
    color: '#7c3aed',
    strokeColor: '#6d28d9',
    visible: true,
    count: 6,
  },
  {
    id: 'frigate',
    name: '护卫舰',
    color: '#0891b2',
    strokeColor: '#0e7490',
    visible: true,
    count: 8,
  },
  {
    id: 'aircraft',
    name: '飞机',
    color: '#e11d48',
    strokeColor: '#be185d',
    visible: false,
    count: 2,
  },
]);

const trackTypes = ref([
  { id: 'primary', name: '主航迹', color: '#60a5fa', style: 'solid', visible: true },
  { id: 'secondary', name: '副航迹', color: '#34d399', style: 'dashed', visible: true },
  { id: 'predicted', name: '预测航迹', color: '#fbbf24', style: 'dotted', visible: false },
]);

const zoneTypes = ref([
  { id: 'patrol', name: '巡逻区', color: '#fbbf24', visible: true },
  { id: 'restricted', name: '禁航区', color: '#ef4444', visible: true },
  { id: 'safe', name: '安全区', color: '#22c55e', visible: false },
]);

// 折叠图标样式类
const collapseIconClasses = computed(() => [
  'text-text-muted transition-transform duration-fast',
  isCollapsed.value && '-rotate-90',
]);

// 获取符号样式类
const getSymbolClasses = (type, visible) => {
  const baseClasses = ['flex-shrink-0 border-2 transition-all duration-fast shadow-sm'];

  if (!visible) {
    baseClasses.push('opacity-30 grayscale');
  } else {
    baseClasses.push('shadow-sm');
  }

  const typeClasses = {
    circle: 'w-3.5 h-3.5 rounded-full',
    square: 'w-3.5 h-3.5 rounded-sm',
    line: 'w-4 h-0.5 border-t-2 border-l-0 border-r-0 border-b-0 rounded-sm',
    zone: 'w-3 h-2.5 border-dashed bg-transparent rounded-xs',
  };

  return [...baseClasses, typeClasses[type] || typeClasses.circle];
};

// 获取名称样式类
const getNameClasses = visible => [
  'text-sm font-medium transition-all duration-fast flex-1',
  visible ? 'text-text-primary' : 'text-text-muted opacity-60 line-through',
];

// Collapse 面板配置
const collapseItems = computed(() => [
  {
    key: 'targets',
    label: '目标类型',
  },
  {
    key: 'platforms',
    label: '平台类型',
  },
  {
    key: 'tracks',
    label: '航迹线',
  },
  {
    key: 'zones',
    label: '控制区域',
  },
]);

// 事件
const emit = defineEmits([
  'target-type-toggle',
  'platform-type-toggle',
  'track-type-toggle',
  'zone-type-toggle',
]);

// 方法
const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value;
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
</script>
