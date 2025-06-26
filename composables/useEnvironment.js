import { ref } from 'vue'

export const useEnvironment = () => {
  const environmentData = ref([
    { label: '海面状况', value: '良好' },
    { label: '能见度', value: '15海里' },
    { label: '风向风速', value: '东南风 3级' },
    { label: '潮汐状况', value: '涨潮' },
    { label: '水温', value: '18°C' }
  ])

  return {
    environmentData
  }
} 