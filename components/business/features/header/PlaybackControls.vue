<template>
  <div class="flex h-8 items-center">
    <div class="flex h-full min-w-[640px] items-center gap-md">
      <!-- 播放控制按钮 -->
      <div class="flex items-center gap-xs">
        <Button
          size="small"
          square
          :variant="isPlaying ? 'success' : 'default'"
          :disabled="!isProjectLoaded || isLoading"
          @click="togglePlayPause"
        >
          <Icon :name="isPlaying ? 'heroicons:pause' : 'heroicons:play'" />
        </Button>
        <Button
          size="small"
          square
          variant="danger"
          :disabled="!isProjectLoaded || isLoading"
          @click="stop"
        >
          <Icon name="heroicons:stop" />
        </Button>
      </div>

      <!-- 进度条区域 -->
      <div class="flex min-w-0 flex-1 items-center">
        <Progress
          v-model="currentProgress"
          :min="0"
          :max="totalDuration"
          variant="primary"
          size="medium"
          :clickable="isProjectLoaded && !isLoading"
          show-thumb
          show-time
          :start-time="startTime"
          :current-time="currentTime"
          class="w-full"
          @change="onProgressChange"
        />
      </div>

      <!-- 倍速控制 -->
      <div class="flex items-center gap-xs">
        <span class="whitespace-nowrap text-text-secondary">倍速</span>
        <Select
          v-model="playbackSpeed"
          :options="speedOptions"
          variant="default"
          @change="onSpeedChange"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import Button from '~/components/base/Button.vue';
import Select from '~/components/input/Select.vue';
import Progress from '~/components/display/Progress.vue';

// 使用composables
const { isProjectLoaded, isLoading, testConnection, currentProject } = useProject();

// 回放状态（暂时保持原有逻辑，后续会用usePlayback替换）
const isPlaying = ref(false);
const playbackSpeed = ref('1');
const currentProgress = ref(0); // 当前进度（分钟）
const totalDuration = ref(0); // 总时长（分钟）

// 倍速选项（根据文档修改移除了低倍速）
const speedOptions = ref([
  { value: '1', label: '×1' },
  { value: '2', label: '×2' },
  { value: '4', label: '×4' },
  { value: '8', label: '×8' },
  { value: '16', label: '×16' },
]);

// 时间显示
const startTime = ref('00:00');
const currentTime = ref('00:00');

// 从ISO时间字符串格式化为显示时间
const formatTimeFromISO = (isoString: string): string => {
  try {
    const date = new Date(isoString);
    return date.toTimeString().substring(0, 8); // HH:MM:SS
  } catch (err) {
    return '00:00:00';
  }
};

// 播放/暂停切换
const togglePlayPause = () => {
  if (!isProjectLoaded.value) return;

  // TODO: 调用后端播放控制命令
  isPlaying.value = !isPlaying.value;
  console.log(isPlaying.value ? '开始播放' : '暂停播放');
};

// 停止
const stop = () => {
  if (!isProjectLoaded.value) return;

  // TODO: 调用后端停止命令
  isPlaying.value = false;
  currentProgress.value = 0;
  currentTime.value = startTime.value;
  console.log('停止播放');
};

// 进度条变化
const onProgressChange = (value: number) => {
  if (!isProjectLoaded.value) return;

  currentProgress.value = value;
  // TODO: 调用后端seek命令
  console.log('跳转到进度:', value);
};

// 倍速变化
const onSpeedChange = (option: { value: string; label: string }) => {
  playbackSpeed.value = option.value;
  // TODO: 调用后端变速命令
  console.log('倍速设置为:', option.label);
};

// 监听工程状态变化，更新播放信息
watch(currentProject, newProject => {
  if (newProject) {
    // 更新播放相关信息
    totalDuration.value = newProject.totalDuration / 1_000_000_000 / 60; // 转换为分钟
    currentProgress.value = 0;
    isPlaying.value = false;

    // 更新时间显示
    startTime.value = formatTimeFromISO(newProject.startTime);
    currentTime.value = startTime.value;

    console.log('工程已加载，更新播放控制信息');
  } else {
    // 工程关闭，重置状态
    totalDuration.value = 0;
    currentProgress.value = 0;
    isPlaying.value = false;
    startTime.value = '00:00';
    currentTime.value = '00:00';

    console.log('工程已关闭，重置播放控制状态');
  }
});

// 组件挂载时测试连接
onMounted(async () => {
  const isConnected = await testConnection();
  if (isConnected) {
    console.log('✅ 后端连接正常');
  } else {
    console.error('❌ 后端连接失败');
  }
});
</script>
