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
  const zoomLevel = ref(1); // 缩放级别

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

  // 播放定时器
  let playbackTimer: number | null = null;

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

  const setZoom = (level: number) => {
    zoomLevel.value = Math.max(0.1, Math.min(level, 10));
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
    // 设置30分钟的测试时长
    totalDuration.value = 30 * 60 * 1000; // 30分钟 = 1800秒 = 1,800,000毫秒
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
            startTime: 2 * 60 * 1000, // 2分钟
            title: '信号强度低',
            description: '接收信号强度降到临界值',
          },
          {
            id: '782-ITC-error-1',
            type: 'error',
            startTime: 8 * 60 * 1000, // 8分钟
            endTime: 8.5 * 60 * 1000, // 8分30秒
            title: '连接中断',
            description: '与指挥中心连接中断30秒',
          },
          {
            id: '782-ITC-info-1',
            type: 'info',
            startTime: 15 * 60 * 1000, // 15分钟
            title: '状态更新',
            description: '系统自检完成，状态正常',
          },
        ],
        activeSegments: [
          { startTime: 0, endTime: 5 * 60 * 1000 }, // 0-5分钟
          { startTime: 7 * 60 * 1000, endTime: 25 * 60 * 1000 }, // 7-25分钟
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
            startTime: 3 * 60 * 1000, // 3分钟
            title: 'GPS精度下降',
            description: '定位精度从3米降至8米',
          },
          {
            id: '155-3X1-error-1',
            type: 'error',
            startTime: 12 * 60 * 1000, // 12分钟
            title: '传感器故障',
            description: '高度传感器读数异常',
          },
          {
            id: '155-3X1-success-1',
            type: 'success',
            startTime: 20 * 60 * 1000, // 20分钟
            endTime: 22 * 60 * 1000, // 22分钟
            title: '维护完成',
            description: '传感器校准完成，系统恢复正常',
          },
        ],
        activeSegments: [
          { startTime: 1 * 60 * 1000, endTime: 18 * 60 * 1000 }, // 1-18分钟
          { startTime: 20 * 60 * 1000, endTime: 30 * 60 * 1000 }, // 20-30分钟
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
            startTime: 1 * 60 * 1000, // 1分钟
            title: '系统启动',
            description: '平台162-Y-8上线',
          },
          {
            id: '162-Y-8-warning-1',
            type: 'warning',
            startTime: 6 * 60 * 1000, // 6分钟
            title: '燃油警告',
            description: '燃油储量低于20%',
          },
          {
            id: '162-Y-8-error-1',
            type: 'error',
            startTime: 10 * 60 * 1000, // 10分钟
            title: '动力系统故障',
            description: '主引擎过热，系统自动关闭',
          },
        ],
        activeSegments: [
          { startTime: 0.5 * 60 * 1000, endTime: 9 * 60 * 1000 }, // 0.5-9分钟
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
            startTime: 4 * 60 * 1000, // 4分钟
            title: '航线更新',
            description: '接收到新的航行指令',
          },
          {
            id: '206-265A-warning-1',
            type: 'warning',
            startTime: 14 * 60 * 1000, // 14分钟
            title: '天气警告',
            description: '前方海域有强风警报',
          },
          {
            id: '206-265A-warning-2',
            type: 'warning',
            startTime: 25 * 60 * 1000, // 25分钟
            title: '设备温度高',
            description: '雷达设备温度过高',
          },
        ],
        activeSegments: [
          { startTime: 0, endTime: 12 * 60 * 1000 }, // 0-12分钟
          { startTime: 16 * 60 * 1000, endTime: 30 * 60 * 1000 }, // 16-30分钟
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
    zoomLevel,
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
    setZoom,
    startDragging,
    stopDragging,
    initializePlatforms,
    initializeTestData,
    addEvent,
    getEventsAtTime,
    formatTime,
    cleanup,
  };

  return timelineState;
};
