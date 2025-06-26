<template>
  <div class="screen-warning-overlay">
    <div class="warning-content">
      <div class="warning-icon">
        <Icon name="heroicons:exclamation-triangle" />
      </div>
      <h2 class="warning-title">屏幕尺寸不合适</h2>
      <div class="warning-message">
        <p>当前屏幕尺寸：{{ width }} × {{ height }}</p>
        <p>建议最小尺寸：{{ MIN_WIDTH }} × {{ MIN_HEIGHT }}</p>
        <p class="suggestion">请最大化浏览器窗口或使用更大的屏幕以获得最佳体验</p>
      </div>
      <div class="warning-actions">
        <button class="action-btn primary" @click="checkFullscreen">
          <Icon name="heroicons:arrows-pointing-out" />
          尝试全屏
        </button>
        <button class="action-btn secondary" @click="forceShow = true">
          仍要继续查看
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  width: Number,
  height: Number,
  MIN_WIDTH: Number,
  MIN_HEIGHT: Number
})

const emit = defineEmits(['force-show'])

const forceShow = ref(false)

const checkFullscreen = () => {
  if (document.documentElement.requestFullscreen) {
    document.documentElement.requestFullscreen()
  } else if (document.documentElement.webkitRequestFullscreen) {
    document.documentElement.webkitRequestFullscreen()
  } else if (document.documentElement.msRequestFullscreen) {
    document.documentElement.msRequestFullscreen()
  }
}

watch(forceShow, (value) => {
  if (value) {
    emit('force-show')
  }
})
</script>

<style scoped>
.screen-warning-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.95);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  font-family: 'Microsoft YaHei', sans-serif;
}

.warning-content {
  background: #1a202c;
  color: #e2e8f0;
  padding: 40px;
  border-radius: 12px;
  text-align: center;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  border: 1px solid #2d3748;
  max-width: 500px;
  margin: 20px;
}

.warning-icon {
  font-size: 48px;
  color: #f59e0b;
  margin-bottom: 20px;
}

.warning-title {
  font-size: 24px;
  color: #f1f5f9;
  margin-bottom: 20px;
  font-weight: bold;
}

.warning-message {
  margin-bottom: 30px;
  line-height: 1.6;
}

.warning-message p {
  margin-bottom: 8px;
  color: #cbd5e0;
}

.suggestion {
  color: #f59e0b !important;
  font-weight: bold;
  margin-top: 16px !important;
}

.warning-actions {
  display: flex;
  gap: 12px;
  justify-content: center;
  flex-wrap: wrap;
}

.action-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: bold;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 140px;
  justify-content: center;
}

.action-btn.primary {
  background: #3b82f6;
  color: white;
}

.action-btn.primary:hover {
  background: #2563eb;
  transform: translateY(-1px);
}

.action-btn.secondary {
  background: #374151;
  color: #e5e7eb;
  border: 1px solid #4b5563;
}

.action-btn.secondary:hover {
  background: #4b5563;
  border-color: #60a5fa;
}
</style> 