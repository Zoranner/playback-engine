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

    <!-- 时间标签 -->
    <div class="absolute inset-0">
      <div
        v-for="label in timeLabels"
        :key="`label-${label.time}`"
        class="absolute top-1 -translate-x-1/2 transform font-mono text-xs text-text-secondary"
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
      class="pointer-events-none absolute -top-6 -translate-x-1/2 transform rounded border border-border bg-background-primary px-1 py-0.5 font-mono text-xs text-text-primary"
      :style="{ left: `${hoverPosition}%` }"
    >
      {{ formatTime(hoverTime) }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const { totalDuration, zoomLevel, seekTo, formatTime } = useTimeline();

const rulerRef = ref<HTMLElement>();
const hoverTime = ref<number | null>(null);
const hoverPosition = ref(0);

// 计算刻度间隔（根据缩放级别动态调整）
const tickInterval = computed(() => {
  // 基础间隔（毫秒）
  const baseInterval = 60000; // 1分钟
  const zoomFactor = zoomLevel.value;

  if (zoomFactor >= 4) {
    return 10000; // 10秒
  } else if (zoomFactor >= 2) {
    return 30000; // 30秒
  } else if (zoomFactor >= 1) {
    return 60000; // 1分钟
  } else if (zoomFactor >= 0.5) {
    return 300000; // 5分钟
  } else {
    return 600000; // 10分钟
  }
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
  const minorInterval = majorInterval / 5;

  for (let time = 0; time <= totalDuration.value; time += minorInterval) {
    // 跳过主要刻度位置
    if (time % majorInterval !== 0) {
      const position = (time / totalDuration.value) * 100;
      ticks.push({ time, position });
    }
  }

  return ticks;
});

// 时间标签
const timeLabels = computed(() => {
  if (totalDuration.value === 0) return [];

  const labels = [];
  const interval = tickInterval.value;

  for (let time = 0; time <= totalDuration.value; time += interval) {
    const position = (time / totalDuration.value) * 100;
    const text = formatTime(time);
    labels.push({ time, position, text });
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
