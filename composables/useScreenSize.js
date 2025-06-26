import { ref, computed, onMounted, onUnmounted } from 'vue'

export const useScreenSize = () => {
  const width = ref(0)
  const height = ref(0)
  
  // 定义最小尺寸要求
  const MIN_WIDTH = 800
  const MIN_HEIGHT = 600
  
  const isScreenTooSmall = computed(() => {
    return width.value < MIN_WIDTH || height.value < MIN_HEIGHT
  })
  
  const updateSize = () => {
    width.value = window.innerWidth
    height.value = window.innerHeight
  }
  
  onMounted(() => {
    updateSize()
    window.addEventListener('resize', updateSize)
  })
  
  onUnmounted(() => {
    window.removeEventListener('resize', updateSize)
  })
  
  return {
    width,
    height,
    isScreenTooSmall,
    MIN_WIDTH,
    MIN_HEIGHT
  }
} 