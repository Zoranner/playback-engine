<template>
  <div
    class="timeline-container flex flex-col overflow-hidden rounded-lg border border-border bg-background-primary shadow-lg"
  >
    <!-- 时间轴头部控制区域 -->
    <TimelineHeader />

    <!-- 时间轴主体区域 -->
    <div class="flex max-h-[400px] min-h-[120px] flex-col overflow-hidden">
      <!-- 时间标尺 -->
      <TimelineRuler />

      <!-- 轨道区域 -->
      <div class="flex-1 overflow-y-auto">
        <TimelineTracks />
      </div>
    </div>

    <!-- 底部状态栏（可选） -->
    <div
      v-if="showStatusBar"
      class="flex h-6 items-center justify-between border-t border-border bg-background-tertiary px-md text-xs text-text-secondary"
    >
      <div class="flex items-center gap-md">
        <span>缩放: {{ Math.round(zoomLevel * 100) }}%</span>
        <span>平台: {{ activePlatformCount }}/{{ totalPlatformCount }}</span>
        <span v-if="totalEventCount > 0">事件: {{ totalEventCount }}</span>
      </div>

      <div class="flex items-center gap-md">
        <span v-if="currentProject">{{ currentProject.name }}</span>
        <span v-else>演示模式</span>
        <span>{{ currentTimeString }} / {{ totalDurationString }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch, onMounted, onUnmounted } from 'vue';
import TimelineHeader from './TimelineHeader.vue';
import TimelineRuler from './TimelineRuler.vue';
import TimelineTracks from './TimelineTracks.vue';
import Button from '~/components/base/Button.vue';

interface Props {
  showStatusBar?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  showStatusBar: true,
});

// 使用时间轴状态管理
const {
  zoomLevel,
  platforms,
  allEvents,
  currentTimeString,
  totalDurationString,
  initializePlatforms,
  initializeTestData,
  cleanup,
} = useTimeline();

// 使用项目状态管理
const { currentProject } = useProject();

// 计算属性
const activePlatformCount = computed(() => {
  return platforms.value.filter(platform => platform.isActive).length;
});

const totalPlatformCount = computed(() => {
  return platforms.value.length;
});

const totalEventCount = computed(() => {
  return allEvents.value.length;
});

// 手动加载测试数据
const loadTestData = () => {
  initializeTestData();
  console.log('已加载测试数据，包含4个平台和多个事件');
  console.log('平台数据:', platforms.value);
  console.log('总时长:', totalDurationString.value);
  console.log('所有事件:', allEvents.value);
};

// 监听项目变化，初始化时间轴数据
watch(
  currentProject,
  newProject => {
    initializePlatforms(newProject);
  },
  { immediate: true }
);

// 组件挂载时的初始化
onMounted(() => {
  console.log('时间轴播放器组件已挂载');

  // 不自动加载测试数据，让用户手动点击按钮加载
  console.log('请点击"加载测试数据"按钮来测试功能');
});

// 组件卸载时清理
onUnmounted(() => {
  cleanup();
  console.log('时间轴播放器组件已卸载，定时器已清理');
});
</script>

<style scoped>
/* 确保时间轴容器可以被进度指示器正确识别 */
.timeline-container {
  position: relative;
}

/* 滚动条样式优化 */
.timeline-container ::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

.timeline-container ::-webkit-scrollbar-track {
  background: theme('colors.background.tertiary');
}

.timeline-container ::-webkit-scrollbar-thumb {
  background: theme('colors.border.light');
  border-radius: 3px;
}

.timeline-container ::-webkit-scrollbar-thumb:hover {
  background: theme('colors.border.DEFAULT');
}
</style>
