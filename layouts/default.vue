<template>
  <div
    class="relative flex h-screen w-screen flex-col overflow-hidden bg-background-primary font-main text-body text-text-primary"
  >
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
    <div
      class="flex h-full w-full flex-col overflow-hidden"
      :class="{
        'relative origin-top-left scale-[0.8] border-2 border-warning':
          isScreenTooSmall && forceShow,
      }"
    >
      <!-- 屏幕过小警告条（替代伪元素） -->
      <div
        v-if="isScreenTooSmall && forceShow"
        class="fixed inset-x-0 top-0 z-modal bg-warning px-xs py-xs text-center font-semibold text-background-primary shadow-md"
      >
        ⚠️ 当前屏幕尺寸过小，可能影响显示效果
      </div>

      <div class="flex flex-1 flex-col overflow-hidden">
        <slot />
      </div>
      <StatusBar />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useScreenSize } from '~/composables/useScreenSize';
import ScreenSizeWarning from '~/components/feedback/ScreenSizeWarning.vue';
import StatusBar from '~/components/business/StatusBar.vue';

const { width, height, isScreenTooSmall, MIN_WIDTH, MIN_HEIGHT } = useScreenSize();
const forceShow = ref(false);
</script>
