<template>
  <div
    ref="rulerRef"
    class="relative h-8 cursor-pointer select-none overflow-hidden border-b border-border bg-background-tertiary"
    @click="onRulerClick"
  >
    <!-- 时间刻度背景 -->
    <div class="absolute inset-0">
      <!-- 主要刻度（较长的线） -->
      <div
        v-for="tick in majorTicks"
        :key="`major-${tick.time}`"
        class="absolute top-0 h-full w-px bg-border-light"
        :style="{ left: `${tick.position}%` }"
      />

      <!-- 次要刻度（较短的线） -->
      <div
        v-for="tick in minorTicks"
        :key="`minor-${tick.time}`"
        class="absolute top-4 h-4 w-px bg-border"
        :style="{ left: `${tick.position}%` }"
      />
    </div>

    <!-- 日期标签 -->
    <div class="absolute inset-0">
      <div
        v-for="label in dateLabels"
        :key="`date-${label.time}`"
        class="absolute top-1/2 -translate-x-1/2 -translate-y-1/2 transform font-mono text-xs text-text-secondary"
        :style="{ left: `${label.position}%` }"
      >
        {{ label.text }}
      </div>
    </div>

    <!-- 鼠标悬停时的时间提示 -->
    <div
      v-if="hoverTime !== null"
      class="pointer-events-none absolute top-0 h-full w-px bg-primary"
      :style="{ left: `${hoverPosition}%` }"
    />
    <div
      v-if="hoverTime !== null"
      class="pointer-events-none fixed z-[9999] transform rounded border border-border bg-background-primary px-1 py-0.5 font-mono text-xs text-text-primary shadow-lg"
      :style="hoverTooltipStyle"
    >
      {{ formatDate(hoverTime, 'MM-DD') }}
      {{ formatTimeOnly(hoverTime) }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const { totalDuration, seekTo, formatTime, formatTimeSmart, formatDate, formatTimeOnly } =
  useTimeline();

const rulerRef = ref<HTMLElement>();
const hoverTime = ref<number | null>(null);
const hoverPosition = ref(0);
const hoverX = ref(0); // 鼠标X坐标

// 计算悬浮提示的位置样式
const hoverTooltipStyle = computed(() => {
  if (hoverTime.value === null || !rulerRef.value) return {};

  const rect = rulerRef.value.getBoundingClientRect();
  const left = hoverPosition.value;
  const tooltipLeft = rect.left + (rect.width * left) / 100;

  let transform = '-translate-x-1/2';
  const finalLeft = tooltipLeft;

  // 如果靠近右边缘，调整定位避免被截断
  if (left > 80) {
    transform = '-translate-x-full';
  } else if (left < 20) {
    transform = 'translate-x-0';
  }

  // 计算垂直位置（标尺下方）
  const tooltipTop = rect.bottom + 8; // 8px 间距

  return {
    left: `${finalLeft}px`,
    top: `${tooltipTop}px`,
    transform,
  };
});

// 主要刻度间隔（固定为1天）
const tickInterval = computed(() => {
  return 24 * 60 * 60 * 1000; // 1天 = 24小时 = 86,400,000毫秒
});

// 主要刻度
const majorTicks = computed(() => {
  if (totalDuration.value === 0) return [];

  const ticks = [];
  const interval = tickInterval.value;

  for (let time = 0; time <= totalDuration.value; time += interval) {
    const position = (time / totalDuration.value) * 100;
    ticks.push({ time, position });
  }

  return ticks;
});

// 次要刻度
const minorTicks = computed(() => {
  if (totalDuration.value === 0) return [];

  const ticks = [];
  const majorInterval = tickInterval.value;

  // 计算主要刻度的数量
  const majorTickCount = Math.floor(totalDuration.value / majorInterval) + 1;

  // 根据主要刻度数量动态计算次要刻度数量（主要刻度越多，次要刻度越少）
  let minorTickCount;
  if (majorTickCount >= 15) {
    // 15个以上主要刻度：2个次要刻度
    minorTickCount = 2;
  } else if (majorTickCount >= 8) {
    // 8-14个主要刻度：4个次要刻度
    minorTickCount = 4;
  } else if (majorTickCount >= 4) {
    // 4-7个主要刻度：8个次要刻度
    minorTickCount = 8;
  } else if (majorTickCount >= 2) {
    // 2-3个主要刻度：16个次要刻度
    minorTickCount = 16;
  } else {
    // 1个主要刻度：24个次要刻度
    minorTickCount = 24;
  }

  // 计算次要刻度间隔
  const minorInterval = majorInterval / minorTickCount;

  for (let time = 0; time <= totalDuration.value; time += minorInterval) {
    // 跳过主要刻度位置
    if (time % majorInterval !== 0) {
      const position = (time / totalDuration.value) * 100;
      ticks.push({ time, position });
    }
  }

  return ticks;
});

// 日期标签
const dateLabels = computed(() => {
  if (totalDuration.value === 0) return [];

  const labels = [];
  const interval = tickInterval.value;

  for (let time = 0; time <= totalDuration.value; time += interval) {
    const position = (time / totalDuration.value) * 100;
    const text = formatDate(time, 'MM-DD');

    // 检查标签是否在可见区域内（留出边距避免被遮挡）
    const isVisible = position >= 8 && position <= 92; // 左右各留8%的边距

    if (isVisible) {
      labels.push({ time, position, text });
    }
  }

  return labels;
});

// 鼠标移动事件处理
const onMouseMove = (event: MouseEvent) => {
  if (!rulerRef.value) return;

  const rect = rulerRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const percentage = (x / rect.width) * 100;

  hoverPosition.value = Math.max(0, Math.min(100, percentage));
  hoverTime.value = (hoverPosition.value / 100) * totalDuration.value;
};

// 鼠标离开事件处理
const onMouseLeave = () => {
  hoverTime.value = null;
};

// 点击事件处理
const onRulerClick = (event: MouseEvent) => {
  if (!rulerRef.value || totalDuration.value === 0) return;

  const rect = rulerRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const percentage = (x / rect.width) * 100;
  const time = (percentage / 100) * totalDuration.value;

  seekTo(time);
};

// 生命周期管理
onMounted(() => {
  if (rulerRef.value) {
    rulerRef.value.addEventListener('mousemove', onMouseMove);
    rulerRef.value.addEventListener('mouseleave', onMouseLeave);
  }
});

onUnmounted(() => {
  if (rulerRef.value) {
    rulerRef.value.removeEventListener('mousemove', onMouseMove);
    rulerRef.value.removeEventListener('mouseleave', onMouseLeave);
  }
});
</script>
