<template>
  <div
    class="fixed inset-0 z-modal flex items-center justify-center bg-background-overlay font-main"
  >
    <div
      class="mx-lg max-w-md rounded-lg border border-border bg-background-card p-xxl text-center text-text-primary shadow-overlay"
    >
      <div class="mb-5 text-warning">
        <Icon name="heroicons:exclamation-triangle" />
      </div>
      <h2 class="mb-5 font-bold text-text-primary">屏幕尺寸不合适</h2>
      <div class="mb-8 leading-loose">
        <p class="mb-2 text-text-muted">当前屏幕尺寸：{{ width }} × {{ height }}</p>
        <p class="mb-2 text-text-muted">建议最小尺寸：{{ minWidth }} × {{ minHeight }}</p>
        <p class="mt-4 font-bold text-warning">请最大化浏览器窗口或使用更大的屏幕以获得最佳体验</p>
      </div>
      <div class="flex flex-wrap justify-center gap-md">
        <Button variant="primary" @click="checkFullscreen">
          <Icon name="heroicons:arrows-pointing-out" />
          尝试全屏
        </Button>
        <Button variant="ghost" @click="forceShow = true"> 仍要继续查看 </Button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import Button from '~/components/base/Button.vue';

defineProps({
  width: { type: Number, default: 0 },
  height: { type: Number, default: 0 },
  minWidth: { type: Number, default: 1200 },
  minHeight: { type: Number, default: 800 },
});

const emit = defineEmits(['force-show']);

const forceShow = ref(false);

const checkFullscreen = () => {
  if (document.documentElement.requestFullscreen) {
    document.documentElement.requestFullscreen();
  } else if (document.documentElement.webkitRequestFullscreen) {
    document.documentElement.webkitRequestFullscreen();
  } else if (document.documentElement.msRequestFullscreen) {
    document.documentElement.msRequestFullscreen();
  }
};

watch(forceShow, value => {
  if (value) {
    emit('force-show');
  }
});
</script>
