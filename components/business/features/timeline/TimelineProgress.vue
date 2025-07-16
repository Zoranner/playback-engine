<template>
  <!-- 进度指示器 -->
  <div
    v-if="totalDuration > 0"
    class="pointer-events-none absolute bottom-0 top-0 z-20"
    :style="{ left: `${progressPosition}%` }"
    style="min-height: 100%"
  >
    <!-- 主要进度线 -->
    <div class="relative h-full w-px bg-primary shadow-lg">
      <!-- 顶部拖拽手柄 -->
      <div
        ref="handleRef"
        class="pointer-events-auto absolute -top-1 left-1/2 h-3 w-3 -translate-x-1/2 transform cursor-grab rounded-full border-2 border-white bg-primary shadow-md transition-transform hover:scale-110 active:cursor-grabbing"
        :class="{ 'scale-110 cursor-grabbing': isDragging }"
        @mousedown="startDrag"
      />

      <!-- 当前时间显示 -->
      <div
        v-if="showTimeTooltip || isDragging"
        class="pointer-events-none absolute -top-8 left-1/2 -translate-x-1/2 transform whitespace-nowrap rounded bg-primary px-2 py-1 font-mono text-xs text-white"
      >
        {{ formatTimeSmart(currentTime, totalDuration) }}
        <!-- 小箭头 -->
        <div class="absolute left-1/2 top-full -mt-px -translate-x-1/2 transform">
          <div class="h-2 w-2 rotate-45 transform bg-primary"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const {
  currentTime,
  totalDuration,
  currentTimeString,
  progressPercentage,
  isDragging,
  startDragging,
  stopDragging,
  seekTo,
  formatTimeSmart,
} = useTimeline();

const handleRef = ref<HTMLElement>();
const showTimeTooltip = ref(false);

// 进度位置（百分比）
const progressPosition = computed(() => {
  return Math.min(100, Math.max(0, progressPercentage.value));
});

// 拖拽相关变量
let dragStartX = 0;
let dragStartTime = 0;
let timelineContainer: HTMLElement | null = null;

// 开始拖拽
const startDrag = (event: MouseEvent) => {
  event.preventDefault();

  // 找到时间轴容器
  const target = event.target as HTMLElement | null;
  timelineContainer = target?.closest?.('.timeline-container') as HTMLElement | null;
  if (!timelineContainer) return;

  startDragging();
  dragStartX = event.clientX;
  dragStartTime = currentTime.value;

  // 添加全局事件监听
  document.addEventListener('mousemove', onDrag);
  document.addEventListener('mouseup', stopDrag);

  // 阻止文本选择
  document.body.style.userSelect = 'none';
};

// 拖拽过程
const onDrag = (event: MouseEvent) => {
  if (!isDragging.value || !timelineContainer) return;

  const deltaX = event.clientX - dragStartX;
  const containerWidth = timelineContainer.offsetWidth;
  const deltaTime = (deltaX / containerWidth) * totalDuration.value;
  const newTime = Math.max(0, Math.min(totalDuration.value, dragStartTime + deltaTime));

  seekTo(newTime);
};

// 停止拖拽
const stopDrag = () => {
  stopDragging();

  // 移除全局事件监听
  document.removeEventListener('mousemove', onDrag);
  document.removeEventListener('mouseup', stopDrag);

  // 恢复文本选择
  document.body.style.userSelect = '';
};

// 鼠标悬停显示时间
const onMouseEnter = () => {
  if (!isDragging.value) {
    showTimeTooltip.value = true;
  }
};

const onMouseLeave = () => {
  showTimeTooltip.value = false;
};

// 生命周期管理
onMounted(() => {
  if (handleRef.value) {
    handleRef.value.addEventListener('mouseenter', onMouseEnter);
    handleRef.value.addEventListener('mouseleave', onMouseLeave);
  }
});

onUnmounted(() => {
  // 清理事件监听
  document.removeEventListener('mousemove', onDrag);
  document.removeEventListener('mouseup', stopDrag);
  document.body.style.userSelect = '';

  if (handleRef.value) {
    handleRef.value.removeEventListener('mouseenter', onMouseEnter);
    handleRef.value.removeEventListener('mouseleave', onMouseLeave);
  }
});
</script>
