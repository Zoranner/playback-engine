<template>
  <div class="status-bar">
    <div class="status-left">
      <div class="status-item">
        <span class="status-indicator status-indicator--online"/>
        系统运行正常
      </div>
      <div class="status-item">
        <span class="text-monospace">{{ connectionStatus }}</span>
      </div>
    </div>
    
    <div class="status-center">
      <div class="status-item">
        <span class="text-caption">坐标:</span>
        <span class="text-monospace">{{ coordinates }}</span>
      </div>
    </div>
    
    <div class="status-right">
      <div class="status-item">
        <span class="text-caption">缩放:</span>
        <span class="text-monospace">{{ zoomLevel }}%</span>
      </div>
      <div class="status-item">
        <span class="text-caption">{{ currentTime }}</span>
      </div>
      <div class="status-item resize-handle">
        <Icon name="heroicons:arrows-pointing-in" size="12" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const currentTime = ref('')
const coordinates = ref('116.3974, 39.9093')
const zoomLevel = ref(100)
const connectionStatus = ref('TCP:CONNECTED')

const updateTime = () => {
  const now = new Date()
  currentTime.value = now.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

let timeInterval
onMounted(() => {
  updateTime()
  timeInterval = setInterval(updateTime, 1000)
})

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval)
  }
})
</script>

<style scoped>
.status-bar {
  height: var(--status-bar-height);
  background: var(--header-bg);
  border-top: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-md);
  font-size: 11px;
  color: var(--text-muted);
  user-select: none;
  flex-shrink: 0;
}

.status-left,
.status-center,
.status-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-lg);
}

.status-center {
  flex: 1;
  justify-content: center;
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  white-space: nowrap;
}

.status-indicator {
  display: inline-flex;
  align-items: center;
  gap: var(--spacing-xs);
  font-size: 11px;
  font-weight: 500;
}

.status-indicator::before {
  content: '';
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
  box-shadow: 0 0 3px currentColor;
}

.status-indicator--online {
  color: var(--success-color);
}

.resize-handle {
  cursor: nw-resize;
  opacity: 0.5;
  transition: opacity var(--transition-fast);
}

.resize-handle:hover {
  opacity: 1;
}

.text-monospace {
  font-family: var(--font-family-mono);
  font-size: 10px;
  color: var(--text-accent);
}

.text-caption {
  color: var(--text-muted);
  font-size: 10px;
}
</style> 