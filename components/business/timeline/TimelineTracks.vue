<template>
  <div class="relative flex flex-col bg-background-primary">
    <!-- 总进度条（当未展开时显示） -->
    <div
      v-if="!isExpanded"
      class="flex h-8 border-b border-border bg-background-primary transition-colors hover:bg-background-secondary"
      style="min-height: 32px"
    >
      <!-- 左侧：总进度标签 -->
      <div class="flex w-20 items-center border-r border-border bg-background-tertiary px-xs">
        <span class="text-xs font-medium text-text-primary">总进度</span>
      </div>

      <!-- 中间：总进度轨道 -->
      <div class="relative flex-1 overflow-hidden" style="min-width: 200px">
        <!-- 背景 -->
        <div class="absolute inset-0 bg-gray-100 dark:bg-gray-800" />

        <!-- 总体活动区间 -->
        <div
          v-for="(segment, index) in overallActiveSegments"
          :key="`overall-${index}`"
          class="absolute top-0 h-full bg-success opacity-60 transition-all duration-300"
          :style="getSegmentStyle(segment)"
        />

        <!-- 所有事件的汇总标记 -->
        <div class="absolute inset-0">
          <TimelineEvent
            v-for="event in allEvents"
            :key="event.id"
            :event="event"
            :total-duration="totalDuration"
          />
        </div>

        <!-- 点击区域 -->
        <div class="absolute inset-0 cursor-pointer" @click="onTrackClick" />
      </div>

      <!-- 右侧：总体状态 -->
      <div class="flex items-center gap-xs border-l border-border bg-background-tertiary px-xs">
        <!-- 活动平台数量 -->
        <div class="flex items-center gap-xs">
          <div class="h-2 w-2 rounded-full bg-success" />
          <span class="text-xs text-text-secondary"
            >{{ activePlatformCount }}/{{ totalPlatformCount }}</span
          >
        </div>

        <!-- 总事件计数 -->
        <div v-if="totalEventCount > 0" class="flex items-center gap-xs">
          <Icon name="heroicons:exclamation-triangle" class="h-3 w-3 text-warning" />
          <span class="text-xs text-text-secondary">{{ totalEventCount }}</span>
        </div>
      </div>
    </div>

    <!-- 展开的平台轨道列表 -->
    <div v-if="isExpanded" class="flex flex-col">
      <TimelineTrack v-for="platform in platforms" :key="platform.id" :platform="platform" />
    </div>

    <!-- 调试信息 -->
    <div v-if="platforms.length === 0" class="p-4 text-center text-red-500">
      没有检测到平台数据，请点击"加载测试数据"按钮
    </div>

    <!-- 调试：显示平台数量 -->
    <div v-else class="border-b p-2 text-xs text-gray-500">
      已加载 {{ platforms.length }} 个平台，总时长 {{ (totalDuration / 60000).toFixed(1) }} 分钟
    </div>

    <!-- 展开/收起提示条 -->
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
        <span>{{ isExpanded ? '收起平台详情' : `展开查看 ${platforms.length} 个平台` }}</span>
        <span class="ml-2 text-green-500"
          >(总时长: {{ (totalDuration / 60000).toFixed(1) }}分钟)</span
        >
      </div>
    </div>

    <!-- 进度指示器（贯穿所有轨道） -->
    <TimelineProgress />
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue';
import TimelineTrack from './TimelineTrack.vue';
import TimelineEvent from './TimelineEvent.vue';
import TimelineProgress from './TimelineProgress.vue';

const { platforms, totalDuration, isExpanded, allEvents, toggleExpanded, seekTo } = useTimeline();

// 总体活动区间（合并所有平台的活动区间）
const overallActiveSegments = computed(() => {
  if (platforms.value.length === 0) return [];

  // 收集所有活动区间
  const allSegments = platforms.value.flatMap(platform => platform.activeSegments || []);

  if (allSegments.length === 0) return [];

  // 合并重叠的区间
  const sortedSegments = allSegments.sort((a, b) => a.startTime - b.startTime);
  const mergedSegments = [];
  let currentSegment = { ...sortedSegments[0] };

  for (let i = 1; i < sortedSegments.length; i++) {
    const segment = sortedSegments[i];

    if (segment.startTime <= currentSegment.endTime) {
      // 区间重叠，合并
      currentSegment.endTime = Math.max(currentSegment.endTime, segment.endTime);
    } else {
      // 区间不重叠，保存当前区间并开始新区间
      mergedSegments.push(currentSegment);
      currentSegment = { ...segment };
    }
  }

  mergedSegments.push(currentSegment);
  return mergedSegments;
});

// 活动平台数量
const activePlatformCount = computed(() => {
  return platforms.value.filter(platform => platform.isActive).length;
});

// 总平台数量
const totalPlatformCount = computed(() => {
  return platforms.value.length;
});

// 总事件数量
const totalEventCount = computed(() => {
  return allEvents.value.length;
});

// 获取区间样式
const getSegmentStyle = (segment: { startTime: number; endTime: number }) => {
  if (totalDuration.value === 0) return { display: 'none' };

  const startPercent = (segment.startTime / totalDuration.value) * 100;
  const endPercent = (segment.endTime / totalDuration.value) * 100;
  const width = endPercent - startPercent;

  return {
    left: `${startPercent}%`,
    width: `${Math.max(width, 0.1)}%`,
  };
};

// 总进度条点击事件
const onTrackClick = (event: MouseEvent) => {
  console.log('点击了总进度条');
  if (totalDuration.value === 0) {
    console.log('总时长为0，无法跳转');
    return;
  }

  const target = event.currentTarget as HTMLElement;
  const rect = target.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const percentage = (x / rect.width) * 100;
  const time = (percentage / 100) * totalDuration.value;

  console.log(`点击位置: ${percentage.toFixed(1)}%, 跳转到: ${time}ms`);
  seekTo(time);
};

// 监听平台数据变化
watch(
  platforms,
  newPlatforms => {
    console.log('TimelineTracks: 平台数据发生变化，数量:', newPlatforms.length);
    if (newPlatforms.length > 0) {
      console.log(
        '平台列表:',
        newPlatforms.map(p => p.name)
      );
    }
  },
  { immediate: true, deep: true }
);
</script>
