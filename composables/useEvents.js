import { ref } from 'vue'

export const useEvents = () => {
  const events = ref([
    { id: 1, time: '14:35:22', title: '新目标发现', description: '雷达发现不明目标', level: 'high' },
    { id: 2, time: '14:33:15', title: '航迹更新', description: '目标743航迹已更新', level: 'normal' },
    { id: 3, time: '14:31:08', title: '通信恢复', description: '与指挥所通信恢复正常', level: 'normal' },
    { id: 4, time: '14:28:45', title: '系统告警', description: '雷达系统出现异常', level: 'warning' },
    { id: 5, time: '14:25:30', title: '目标离开', description: '目标1691已离开监控区域', level: 'low' }
  ])

  return {
    events
  }
} 