<template>
  <div class="playback-controls">
    <div class="controls-container">
      <!-- 播放控制按钮 -->
      <div class="control-buttons">
        <button class="control-btn play-pause-btn" :class="{ 'playing': isPlaying }" @click="togglePlayPause">
          <Icon :name="isPlaying ? 'heroicons:pause' : 'heroicons:play'" />
        </button>
        <button class="control-btn stop-btn" @click="stop">
          <Icon name="heroicons:stop" />
        </button>
      </div>

      <!-- 进度条区域 -->
      <div class="progress-section">
        <span class="time-start text-monospace">{{ startTime }}</span>
        <div class="progress-container">
          <input 
            v-model="currentProgress" 
            type="range" 
            class="progress-bar" 
            :min="0" 
            :max="totalDuration"
            @input="onProgressChange"
          >
          <div class="progress-track"/>
          <div class="progress-fill" :style="{ width: progressPercentage + '%' }"/>
        </div>
        <span class="time-current text-monospace">{{ currentTime }}</span>
      </div>

      <!-- 倍速控制 -->
      <div class="speed-controls">
        <span class="speed-label">倍速</span>
        <select v-model="playbackSpeed" class="speed-selector military-input">
          <option value="1">×1</option>
          <option value="2">×2</option>
          <option value="4">×4</option>
          <option value="8">×8</option>
          <option value="16">×16</option>
        </select>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

// 回放状态
const isPlaying = ref(false)
const playbackSpeed = ref('1')
const currentProgress = ref(30) // 当前进度（分钟）
const totalDuration = ref(120) // 总时长（分钟）

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

.control-btn {
  width: 24px;
  height: 24px;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  font-size: 11px;
}

.control-btn:hover {
  background: var(--tertiary-bg);
  border-color: var(--border-color-light);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.control-btn:active {
  transform: translateY(0);
  box-shadow: var(--shadow-inset);
}

.play-pause-btn.playing {
  background: var(--success-color);
  border-color: var(--success-color);
  color: var(--primary-bg);
}

.play-pause-btn.playing:hover {
  background: #059669;
}

.stop-btn {
  background: var(--danger-color);
  border-color: var(--danger-color);
  color: var(--primary-bg);
}

.stop-btn:hover {
  background: #dc2626;
}

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