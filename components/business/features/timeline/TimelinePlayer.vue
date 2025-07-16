<template>
  <GroupBox
    title="回放控制"
    :padded="false"
  >
    <div class="timeline-container relative flex flex-col overflow-hidden">
      <!-- 时间轴头部控制区域 -->
      <TimelineHeader />

      <!-- 时间轴主体区域 -->
      <div class="relative flex flex-col">
        <!-- 时间轴内容区域 -->
        <div class="flex flex-1">
          <!-- 左侧：标题区域 -->
          <div class="w-20 flex-shrink-0 border-r border-border bg-background-tertiary">
            <!-- 时间标尺标题占位 -->
            <div class="h-8 border-b border-border"></div>

            <!-- 轨道标题列表 -->
            <div class="flex flex-col">
              <!-- 总进度标题 -->
              <div class="flex h-8 items-center border-b border-border px-xs">
                <span class="text-xs font-medium text-text-primary">总进度</span>
              </div>

              <!-- 平台标题列表（只在展开时显示） -->
              <template v-if="isExpanded">
                <div
                  v-for="platform in platforms"
                  :key="platform.id"
                  class="flex h-8 items-center border-b border-border px-xs"
                >
                  <span class="truncate text-xs font-medium text-text-primary">{{
                    platform.name
                  }}</span>
                </div>
              </template>
            </div>
          </div>

          <!-- 右侧：时间轨道区域 -->
          <div class="relative flex-1 overflow-hidden">
            <!-- 时间标尺 -->
            <TimelineRuler />

            <!-- 轨道区域 -->
            <div class="flex-1 overflow-y-auto">
              <TimelineTracks />
            </div>

            <!-- 进度指示器（只贯穿时间轨道区域） -->
            <TimelineProgress />
          </div>
        </div>

        <!-- 展开/收起提示条（贯穿整个时间轴区域） -->
        <div
          v-if="platforms.length > 0"
          class="flex h-6 cursor-pointer items-center justify-center border-b border-border bg-background-tertiary transition-colors hover:bg-background-secondary"
          @click="toggleExpanded"
        >
          <div class="flex items-center gap-xs text-xs text-text-secondary">
            <Icon
              :name="isExpanded ? 'heroicons:chevron-up' : 'heroicons:chevron-down'"
              class="h-3 w-3"
            />
            <span>{{ isExpanded ? '收起平台详情' : '展开平台详情' }}</span>
          </div>
        </div>
      </div>
    </div>
  </GroupBox>
</template>

<script setup lang="ts">
import { watch, onMounted, onUnmounted } from 'vue';
import TimelineHeader from './TimelineHeader.vue';
import TimelineRuler from './TimelineRuler.vue';
import TimelineTracks from './TimelineTracks.vue';
import TimelineProgress from './TimelineProgress.vue';
import GroupBox from '~/components/display/GroupBox.vue';

// 使用时间轴状态管理
const { platforms, isExpanded, toggleExpanded, initializePlatforms, initializeTestData, cleanup } =
  useTimeline();

// 使用项目状态管理
const { currentProject } = useProject();

// 手动加载测试数据
const loadTestData = () => {
  initializeTestData();
  console.log('已加载测试数据，包含4个平台和多个事件');
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
/* 滚动条样式优化 - 无法通过TailwindCSS内联样式实现 */
.timeline-container ::-webkit-scrollbar {
  width: 6px;
  height: 60px;
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
