import { ref, computed, watch } from 'vue';

// 播放状态类型
export type PlaybackState = 'playing' | 'paused' | 'stopped';

// 全局状态，确保所有组件共享同一个实例
let timelineState: any = null;

// 事件类型
export interface TimelineEvent {
  id: string;
  type: 'warning' | 'error' | 'info' | 'success';
  startTime: number; // 相对时间（毫秒）
  endTime?: number; // 结束时间，用于区间事件
  title: string;
  description?: string;
}

// 平台数据类型
export interface Platform {
  id: string;
  name: string;
  isActive: boolean;
  events: TimelineEvent[];
  // 活动区间（绿色部分）
  activeSegments: Array<{
    startTime: number;
    endTime: number;
  }>;
}

export const useTimeline = () => {
  // 如果已经存在实例，直接返回
  if (timelineState) {
    return timelineState;
  }

  // 基础播放状态
  const playbackState = ref<PlaybackState>('stopped');
  const currentTime = ref(0); // 当前时间（毫秒）
  const totalDuration = ref(0); // 总时长（毫秒）
  const playbackSpeed = ref(1); // 播放倍速

  // 时间轴UI状态
  const isExpanded = ref(false); // 是否展开显示所有平台
  const isDragging = ref(false); // 是否正在拖拽进度条

  // 平台数据
  const platforms = ref<Platform[]>([]);

  // 倍速选项
  const speedOptions = [
    { value: 1, label: '×1' },
    { value: 2, label: '×2' },
    { value: 4, label: '×4' },
    { value: 8, label: '×8' },
    { value: 16, label: '×16' },
  ];

  // 计算属性
  const isPlaying = computed(() => playbackState.value === 'playing');
  const isPaused = computed(() => playbackState.value === 'paused');
  const isStopped = computed(() => playbackState.value === 'stopped');

  // 进度百分比（0-100）
  const progressPercentage = computed(() => {
    if (totalDuration.value === 0) return 0;
    return (currentTime.value / totalDuration.value) * 100;
  });

  // 当前时间字符串
  const currentTimeString = computed(() => {
    return formatTime(currentTime.value);
  });

  // 总时长字符串
  const totalDurationString = computed(() => {
    return formatTime(totalDuration.value);
  });

  // 所有事件（合并所有平台的事件）
  const allEvents = computed(() => {
    return platforms.value.flatMap(platform => platform.events);
  });

  // 时间格式化函数
  const formatTime = (milliseconds: number): string => {
    const totalSeconds = Math.floor(milliseconds / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    if (hours > 0) {
      return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
    }
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  };

  // 智能时间格式化函数（始终使用日期时间格式）
  const formatTimeSmart = (milliseconds: number, totalDuration: number): string => {
    // 设置基准时间为2024年1月1日 00:00:00
    const baseDate = new Date('2024-01-01T00:00:00');
    const targetDate = new Date(baseDate.getTime() + milliseconds);

    const year = targetDate.getFullYear();
    const month = (targetDate.getMonth() + 1).toString().padStart(2, '0');
    const day = targetDate.getDate().toString().padStart(2, '0');
    const hours = targetDate.getHours().toString().padStart(2, '0');
    const minutes = targetDate.getMinutes().toString().padStart(2, '0');
    const seconds = targetDate.getSeconds().toString().padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  };

  // 日期格式化函数
  const formatDate = (milliseconds: number, format: string = 'YYYY-MM-DD'): string => {
    // 设置基准时间为2024年1月1日 00:00:00
    const baseDate = new Date('2024-01-01T00:00:00');
    const targetDate = new Date(baseDate.getTime() + milliseconds);

    const year = targetDate.getFullYear();
    const month = (targetDate.getMonth() + 1).toString().padStart(2, '0');
    const day = targetDate.getDate().toString().padStart(2, '0');

    switch (format) {
      case 'MM-DD':
        return `${month}-${day}`;
      case 'MM/DD':
        return `${month}/${day}`;
      case 'YYYY-MM-DD':
      default:
        return `${year}-${month}-${day}`;
    }
  };

  // 时间格式化函数（仅时间部分，处理跨天情况）
  const formatTimeOnly = (milliseconds: number): string => {
    // 设置基准时间为2024年1月1日 00:00:00
    const baseDate = new Date('2024-01-01T00:00:00');
    const targetDate = new Date(baseDate.getTime() + milliseconds);

    const hours = targetDate.getHours().toString().padStart(2, '0');
    const minutes = targetDate.getMinutes().toString().padStart(2, '0');
    const seconds = targetDate.getSeconds().toString().padStart(2, '0');

    return `${hours}:${minutes}:${seconds}`;
  };

  // 播放定时器
  let playbackTimer: ReturnType<typeof setInterval> | null = null;

  // 播放控制方法
  const play = () => {
    if (totalDuration.value === 0) return;

    playbackState.value = 'playing';
    console.log('开始播放');

    // 启动播放定时器（每100ms更新一次）
    if (playbackTimer) clearInterval(playbackTimer);
    playbackTimer = setInterval(() => {
      if (playbackState.value === 'playing') {
        const increment = 100 * playbackSpeed.value; // 100ms * 倍速
        currentTime.value += increment;

        // 如果达到结束时间，自动停止
        if (currentTime.value >= totalDuration.value) {
          currentTime.value = totalDuration.value;
          stop();
        }
      }
    }, 100);

    // TODO: 调用后端播放命令
  };

  const pause = () => {
    playbackState.value = 'paused';

    // 清除定时器
    if (playbackTimer) {
      clearInterval(playbackTimer);
      playbackTimer = null;
    }

    console.log('暂停播放');
    // TODO: 调用后端暂停命令
  };

  const stop = () => {
    playbackState.value = 'stopped';
    currentTime.value = 0;

    // 清除定时器
    if (playbackTimer) {
      clearInterval(playbackTimer);
      playbackTimer = null;
    }

    console.log('停止播放');
    // TODO: 调用后端停止命令
  };

  const togglePlayPause = () => {
    if (isPlaying.value) {
      pause();
    } else {
      play();
    }
  };

  // 进度控制
  const seekTo = (time: number) => {
    currentTime.value = Math.max(0, Math.min(time, totalDuration.value));
    console.log('跳转到时间:', formatTime(currentTime.value));
    // TODO: 调用后端seek命令
  };

  const seekToPercentage = (percentage: number) => {
    const time = (percentage / 100) * totalDuration.value;
    seekTo(time);
  };

  // 倍速控制
  const setPlaybackSpeed = (speed: number) => {
    playbackSpeed.value = speed;
    console.log('设置倍速:', speed);

    // 如果正在播放，重新启动定时器以应用新的倍速
    if (playbackState.value === 'playing') {
      if (playbackTimer) {
        clearInterval(playbackTimer);
        playbackTimer = null;
      }
      play(); // 重新开始播放以应用新倍速
    }

    // TODO: 调用后端倍速命令
  };

  // 时间轴控制
  const toggleExpanded = () => {
    isExpanded.value = !isExpanded.value;
  };

  // 拖拽控制
  const startDragging = () => {
    isDragging.value = true;
  };

  const stopDragging = () => {
    isDragging.value = false;
  };

  // 初始化测试数据
  const initializeTestData = () => {
    // 设置7天的测试时长（测试跨日期显示）
    totalDuration.value = 7 * 24 * 60 * 60 * 1000; // 7天 = 604800秒 = 604,800,000毫秒
    currentTime.value = 0;
    playbackState.value = 'stopped';

    // 创建4个测试平台
    platforms.value = [
      {
        id: '782-ITC',
        name: '782-ITC',
        isActive: true,
        events: [
          {
            id: '782-ITC-warning-1',
            type: 'warning',
            startTime: 6 * 60 * 60 * 1000, // 6小时
            title: '信号强度低',
            description: '接收信号强度降到临界值',
          },
          {
            id: '782-ITC-error-1',
            type: 'error',
            startTime: 2 * 24 * 60 * 60 * 1000, // 第2天
            endTime: 2 * 24 * 60 * 60 * 1000 + 2 * 60 * 60 * 1000, // 第2天+2小时
            title: '连接中断',
            description: '与指挥中心连接中断2小时',
          },
          {
            id: '782-ITC-info-1',
            type: 'info',
            startTime: 4 * 24 * 60 * 60 * 1000, // 第4天
            title: '状态更新',
            description: '系统自检完成，状态正常',
          },
        ],
        activeSegments: [
          { startTime: 0, endTime: 1 * 24 * 60 * 60 * 1000 }, // 第1天
          { startTime: 1.5 * 24 * 60 * 60 * 1000, endTime: 6 * 24 * 60 * 60 * 1000 }, // 第1.5-6天
        ],
      },
      {
        id: '155-3X1',
        name: '155-3X1',
        isActive: true,
        events: [
          {
            id: '155-3X1-warning-1',
            type: 'warning',
            startTime: 12 * 60 * 60 * 1000, // 12小时
            title: 'GPS精度下降',
            description: '定位精度从3米降至8米',
          },
          {
            id: '155-3X1-error-1',
            type: 'error',
            startTime: 3 * 24 * 60 * 60 * 1000, // 第3天
            title: '传感器故障',
            description: '高度传感器读数异常',
          },
          {
            id: '155-3X1-success-1',
            type: 'success',
            startTime: 5 * 24 * 60 * 60 * 1000, // 第5天
            endTime: 5 * 24 * 60 * 60 * 1000 + 6 * 60 * 60 * 1000, // 第5天+6小时
            title: '维护完成',
            description: '传感器校准完成，系统恢复正常',
          },
        ],
        activeSegments: [
          { startTime: 6 * 60 * 60 * 1000, endTime: 2 * 24 * 60 * 60 * 1000 }, // 6小时-第2天
          { startTime: 2.5 * 24 * 60 * 60 * 1000, endTime: 6 * 24 * 60 * 60 * 1000 }, // 第2.5-6天
        ],
      },
      {
        id: '162-Y-8',
        name: '162-Y-8',
        isActive: false,
        events: [
          {
            id: '162-Y-8-info-1',
            type: 'info',
            startTime: 18 * 60 * 60 * 1000, // 18小时
            title: '系统启动',
            description: '平台162-Y-8上线',
          },
          {
            id: '162-Y-8-warning-1',
            type: 'warning',
            startTime: 1.5 * 24 * 60 * 60 * 1000, // 第1.5天
            title: '燃油警告',
            description: '燃油储量低于20%',
          },
          {
            id: '162-Y-8-error-1',
            type: 'error',
            startTime: 4.5 * 24 * 60 * 60 * 1000, // 第4.5天
            title: '动力系统故障',
            description: '主引擎过热，系统自动关闭',
          },
        ],
        activeSegments: [
          { startTime: 12 * 60 * 60 * 1000, endTime: 2 * 24 * 60 * 60 * 1000 }, // 12小时-第2天
          { startTime: 3 * 24 * 60 * 60 * 1000, endTime: 5 * 24 * 60 * 60 * 1000 }, // 第3-5天
        ],
      },
      {
        id: '206-265A',
        name: '206-265A',
        isActive: true,
        events: [
          {
            id: '206-265A-info-1',
            type: 'info',
            startTime: 24 * 60 * 60 * 1000, // 第1天
            title: '航线更新',
            description: '接收到新的航行指令',
          },
          {
            id: '206-265A-warning-1',
            type: 'warning',
            startTime: 3.5 * 24 * 60 * 60 * 1000, // 第3.5天
            title: '天气警告',
            description: '前方海域有强风警报',
          },
          {
            id: '206-265A-warning-2',
            type: 'warning',
            startTime: 6 * 24 * 60 * 60 * 1000, // 第6天
            title: '设备温度高',
            description: '雷达设备温度过高',
          },
        ],
        activeSegments: [
          { startTime: 0, endTime: 1.5 * 24 * 60 * 60 * 1000 }, // 0-第1.5天
          { startTime: 2 * 24 * 60 * 60 * 1000, endTime: 7 * 24 * 60 * 60 * 1000 }, // 第2-7天
        ],
      },
    ];
  };

  // 初始化平台数据
  const initializePlatforms = (projectData: any) => {
    if (!projectData) {
      // 如果没有项目数据，初始化测试数据
      initializeTestData();
      return;
    }

    // 根据项目数据初始化平台信息
    totalDuration.value = projectData.totalDuration || 0;

    // TODO: 根据实际数据结构解析平台信息
    platforms.value = [
      {
        id: '782-ITC',
        name: '782-ITC',
        isActive: true,
        events: [],
        activeSegments: [
          { startTime: 0, endTime: totalDuration.value * 0.3 },
          { startTime: totalDuration.value * 0.4, endTime: totalDuration.value * 0.9 },
        ],
      },
      {
        id: '155-3X1',
        name: '155-3X1',
        isActive: true,
        events: [],
        activeSegments: [
          { startTime: 0, endTime: totalDuration.value * 0.6 },
          { startTime: totalDuration.value * 0.7, endTime: totalDuration.value },
        ],
      },
      {
        id: '162-Y-8',
        name: '162-Y-8',
        isActive: true,
        events: [],
        activeSegments: [
          { startTime: totalDuration.value * 0.1, endTime: totalDuration.value * 0.8 },
        ],
      },
      {
        id: '206-265A',
        name: '206-265A',
        isActive: true,
        events: [],
        activeSegments: [
          { startTime: 0, endTime: totalDuration.value * 0.4 },
          { startTime: totalDuration.value * 0.6, endTime: totalDuration.value },
        ],
      },
    ];
  };

  // 添加事件
  const addEvent = (platformId: string, event: TimelineEvent) => {
    const platform = platforms.value.find(p => p.id === platformId);
    if (platform) {
      platform.events.push(event);
    }
  };

  // 获取指定时间的事件
  const getEventsAtTime = (time: number) => {
    return allEvents.value.filter(event => {
      if (event.endTime) {
        return time >= event.startTime && time <= event.endTime;
      }
      return Math.abs(time - event.startTime) < 1000; // 1秒误差范围
    });
  };

  // 清理定时器（组件卸载时调用）
  const cleanup = () => {
    if (playbackTimer) {
      clearInterval(playbackTimer);
      playbackTimer = null;
    }
  };

  // 创建状态对象
  timelineState = {
    // 状态
    playbackState,
    currentTime,
    totalDuration,
    playbackSpeed,
    isExpanded,
    isDragging,
    platforms,
    speedOptions,

    // 计算属性
    isPlaying,
    isPaused,
    isStopped,
    progressPercentage,
    currentTimeString,
    totalDurationString,
    allEvents,

    // 方法
    play,
    pause,
    stop,
    togglePlayPause,
    seekTo,
    seekToPercentage,
    setPlaybackSpeed,
    toggleExpanded,
    startDragging,
    stopDragging,
    initializePlatforms,
    initializeTestData,
    addEvent,
    getEventsAtTime,
    formatTime,
    formatTimeSmart,
    formatDate,
    formatTimeOnly,
    cleanup,
  };

  return timelineState;
};
