<template>
  <div class="playback-engine">
    <!-- 屏幕尺寸警告 -->
    <ScreenSizeWarning 
      v-if="isScreenTooSmall && !forceShow"
      :width="width"
      :height="height"
      :min-width="MIN_WIDTH"
      :min-height="MIN_HEIGHT"
      @force-show="forceShow = true"
    />
    
    <!-- 主要内容 -->
    <div class="main-content" :class="{ 'force-small': isScreenTooSmall && forceShow }">
      <div class="content-wrapper">
        <slot />
      </div>
      <StatusBar />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useScreenSize } from '~/composables/useScreenSize'
import ScreenSizeWarning from '~/components/ScreenSizeWarning.vue'
import StatusBar from '~/components/ui/StatusBar.vue'

const { width, height, isScreenTooSmall, MIN_WIDTH, MIN_HEIGHT } = useScreenSize()
const forceShow = ref(false)
</script>

<style scoped>
.playback-engine {
  height: 100vh;
  width: 100vw;
  background: var(--primary-bg);
  color: var(--text-primary);
  font-family: var(--font-family-main);
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.main-content {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.main-content.force-small {
  transform: scale(0.8);
  transform-origin: top left;
  border: 2px solid var(--warning-color);
  position: relative;
}

.main-content.force-small::before {
  content: "⚠️ 当前屏幕尺寸过小，可能影响显示效果";
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  background: var(--warning-color);
  color: var(--primary-bg);
  text-align: center;
  padding: var(--spacing-xs);
  font-size: 12px;
  font-weight: 600;
  z-index: 1000;
  box-shadow: var(--shadow-md);
}
</style> 