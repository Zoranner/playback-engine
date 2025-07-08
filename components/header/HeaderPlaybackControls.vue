<template>
  <div class="playback-controls">
    <div class="controls-container">
      <!-- 播放控制按钮 -->
      <div class="control-buttons">
        <Button 
          size="small" 
          square 
          :variant="isPlaying ? 'success' : 'default'"
          @click="togglePlayPause"
        >
          <Icon :name="isPlaying ? 'heroicons:pause' : 'heroicons:play'" />
        </Button>
        <Button 
          size="small" 
          square 
          variant="danger" 
          @click="stop"
        >
          <Icon name="heroicons:stop" />
        </Button>
      </div>

      <!-- 进度条区域 -->
      <div class="progress-section">
        <span class="time-start text-monospace">{{ startTime }}</span>
        <div class="progress-container">
          <Slider
            v-model="currentProgress"
            :min="0"
            :max="totalDuration"
            variant="primary"
            class="progress-slider"
            @input="onProgressChange"
          />
        </div>
        <span class="time-current text-monospace">{{ currentTime }}</span>
      </div>

      <!-- 倍速控制 -->
      <div class="speed-controls">
        <span class="speed-label">倍速</span>
        <Select
          v-model="playbackSpeed"
          :options="speedOptions"
          size="small"
          variant="ghost"
          class="speed-selector"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import Button from '~/components/ui/Button.vue'
import Select from '~/components/ui/Select.vue'
import Slider from '~/components/ui/Slider.vue'

// 回放状态
const isPlaying = ref(false)
const playbackSpeed = ref('1')
const currentProgress = ref(30) // 当前进度（分钟）
const totalDuration = ref(120) // 总时长（分钟）

// 倍速选项
const speedOptions = ref([
  { value: '1', label: '×1' },
  { value: '2', label: '×2' },
  { value: '4', label: '×4' },
  { value: '8', label: '×8' },
  { value: '16', label: '×16' }
])

// 时间显示
const startTime = ref('14:30')
const currentTime = ref('15:00')

// 计算进度百分比
const progressPercentage = computed(() => {
  return (currentProgress.value / totalDuration.value) * 100
})

// 播放/暂停切换
const togglePlayPause = () => {
  isPlaying.value = !isPlaying.value
}

// 停止
const stop = () => {
  isPlaying.value = false
  currentProgress.value = 0
}

// 进度条变化
const onProgressChange = (event) => {
  currentProgress.value = event.target.value
}
</script>

<style scoped>
.playback-controls {
  display: flex;
  align-items: center;
  height: 32px; /* 固定高度，确保不超过标题栏 */
}

.controls-container {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  min-width: 480px;
  height: 100%;
}

.control-buttons {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

/* 控制按钮现在使用Button组件 */

.progress-section {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  min-width: 0;
}

.time-start,
.time-current {
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
  min-width: 32px;
  text-align: center;
}

.progress-container {
  flex: 1;
  position: relative;
  height: 3px;
  min-width: 100px;
}

.progress-track {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 100%;
  background: var(--secondary-bg);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-color);
}

.progress-bar {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  cursor: pointer;
  z-index: 2;
}

.progress-fill {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: linear-gradient(90deg, var(--info-color), var(--text-accent));
  border-radius: var(--radius-sm);
  transition: width var(--transition-fast);
  box-shadow: 0 0 2px var(--info-color);
  z-index: 1;
}

.speed-controls {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.speed-label {
  font-size: 10px;
  color: var(--text-secondary);
  white-space: nowrap;
}

.speed-selector {
  width: 50px;
  height: 22px;
  font-size: 10px;
  padding: 2px var(--spacing-xs);
  text-align: center;
  text-align-last: center;
}

.speed-selector option {
  background: var(--secondary-bg);
  color: var(--text-primary);
}
</style> 