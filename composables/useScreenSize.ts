import { ref, onMounted, onUnmounted, computed } from 'vue';

export const useScreenSize = () => {
  const MIN_WIDTH = 1280;
  const MIN_HEIGHT = 720;

  const width = ref<number>(window.innerWidth);
  const height = ref<number>(window.innerHeight);

  const isScreenTooSmall = computed<boolean>(() => {
    return width.value < MIN_WIDTH || height.value < MIN_HEIGHT;
  });

  const handleResize = () => {
    width.value = window.innerWidth;
    height.value = window.innerHeight;
  };

  onMounted(() => {
    window.addEventListener('resize', handleResize);
  });

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
  });

  return {
    width,
    height,
    isScreenTooSmall,
    MIN_WIDTH,
    MIN_HEIGHT
  };
}; 