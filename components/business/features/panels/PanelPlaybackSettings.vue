<template>
  <GroupBox
    title="回放设置"
    class="flex-none"
  >
    <!-- 有选中数据集时显示设置 -->
    <div
      v-if="selectedDataset"
      class="-m-sm flex flex-col gap-sm p-sm"
    >
      <!-- 数据集信息 -->
      <div class="mb-sm border-b border-border-divider pb-sm">
        <div class="mb-1 font-semibold text-text-primary">{{ selectedDataset.name }}</div>
        <div class="text-caption text-text-secondary">
          {{ selectedDataset.files.filter(f => f.type === 'pcap').length }} 个 PCAP 文件
        </div>
      </div>

      <!-- 回放模式设置 -->
      <div class="settings-group">
        <div class="mb-xs font-medium text-text-primary">回放模式</div>
        <div class="flex flex-col gap-xs">
          <label
            v-for="mode in playbackModes"
            :key="mode.value"
            class="flex cursor-pointer items-center gap-sm rounded-sm border border-border bg-background-tertiary px-sm py-xs transition-all hover:border-border-light hover:bg-background-panel"
          >
            <input
              v-model="playbackSettings.mode"
              type="radio"
              :value="mode.value"
              class="h-3 w-3 accent-primary"
            />
            <div class="flex-1">
              <div class="text-subtitle">{{ mode.label }}</div>
              <div class="text-caption text-text-muted">{{ mode.description }}</div>
            </div>
            <div class="text-caption text-text-secondary">{{ mode.speed }}</div>
          </label>
        </div>
      </div>

      <!-- 传输设置 -->
      <div class="settings-group">
        <div class="mb-xs font-medium text-text-primary">传输设置</div>

        <!-- 传输类型 -->
        <div class="mb-sm">
          <div class="mb-xs text-caption text-text-secondary">传输类型</div>
          <Select
            v-model="playbackSettings.transportType"
            :options="transportTypes"
            placeholder="选择传输类型"
            size="small"
          />
        </div>

        <!-- 目标地址 -->
        <div class="mb-sm">
          <div class="mb-xs text-caption text-text-secondary">目标地址</div>
          <input
            v-model="playbackSettings.targetAddress"
            type="text"
            class="w-full rounded-sm border border-border bg-background-primary px-sm py-xs text-subtitle transition-colors focus:border-border-active focus:outline-none"
            placeholder="192.168.1.255"
          />
        </div>

        <!-- 目标端口 -->
        <div class="mb-sm">
          <div class="mb-xs text-caption text-text-secondary">目标端口</div>
          <input
            v-model="playbackSettings.targetPort"
            type="number"
            min="1"
            max="65535"
            class="w-full rounded-sm border border-border bg-background-primary px-sm py-xs text-subtitle transition-colors focus:border-border-active focus:outline-none"
            placeholder="12345"
          />
        </div>

        <!-- 缓冲区大小 -->
        <div class="mb-sm">
          <div class="mb-xs text-caption text-text-secondary">缓冲区大小</div>
          <Select
            v-model="playbackSettings.bufferSize"
            :options="bufferSizeOptions"
            placeholder="选择缓冲区大小"
            size="small"
          />
        </div>
      </div>

      <!-- 高级设置 -->
      <div class="settings-group">
        <div class="mb-xs font-medium text-text-primary">高级设置</div>

        <!-- 同步模式 -->
        <div class="mb-sm flex items-center justify-between">
          <div>
            <div class="text-subtitle">严格时间同步</div>
            <div class="text-caption text-text-muted">确保数据包按精确时间戳发送</div>
          </div>
          <Toggle v-model="playbackSettings.strictTiming" />
        </div>

        <!-- 循环播放 -->
        <div class="mb-sm flex items-center justify-between">
          <div>
            <div class="text-subtitle">循环播放</div>
            <div class="text-caption text-text-muted">播放结束后自动重新开始</div>
          </div>
          <Toggle v-model="playbackSettings.loopPlayback" />
        </div>

        <!-- 自动启动UDP广播 -->
        <div class="flex items-center justify-between">
          <div>
            <div class="text-subtitle">自动UDP广播</div>
            <div class="text-caption text-text-muted">回放时自动启动UDP数据广播</div>
          </div>
          <Toggle v-model="playbackSettings.autoUdpBroadcast" />
        </div>
      </div>

      <!-- 数据集统计 -->
      <div class="settings-group">
        <div class="mb-xs font-medium text-text-primary">数据集统计</div>
        <div class="grid grid-cols-2 gap-sm">
          <div class="rounded-sm border border-border bg-background-tertiary p-sm">
            <div class="text-caption text-text-muted">文件总数</div>
            <div class="font-mono text-subtitle text-primary">{{ selectedDataset.files.length }}</div>
          </div>
          <div class="rounded-sm border border-border bg-background-tertiary p-sm">
            <div class="text-caption text-text-muted">总大小</div>
            <div class="font-mono text-subtitle text-primary">{{ formatTotalSize(selectedDataset.files) }}</div>
          </div>
          <div class="rounded-sm border border-border bg-background-tertiary p-sm">
            <div class="text-caption text-text-muted">PCAP文件</div>
            <div class="font-mono text-subtitle text-success">{{ selectedDataset.files.filter(f => f.type === 'pcap').length }}</div>
          </div>
          <div class="rounded-sm border border-border bg-background-tertiary p-sm">
            <div class="text-caption text-text-muted">索引文件</div>
            <div class="font-mono text-subtitle text-info">{{ selectedDataset.files.filter(f => f.type === 'pidx').length }}</div>
          </div>
        </div>
      </div>

      <!-- 操作按钮 -->
      <div class="flex gap-sm">
        <Button
          variant="primary"
          size="small"
          class="flex-1"
          @click="applySettings"
        >
          应用设置
        </Button>
        <Button
          variant="secondary"
          size="small"
          @click="resetSettings"
        >
          重置
        </Button>
      </div>
    </div>

    <!-- 无选中数据集时的提示 -->
    <div
      v-else
      class="flex flex-col items-center justify-center gap-sm py-lg text-center text-text-muted"
    >
      <Icon
        name="heroicons:cog-6-tooth"
        size="24"
        class="opacity-60"
      />
      <div class="text-text-secondary">未选择数据集</div>
      <div class="text-caption opacity-70">请先在工程管理中选择数据集</div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref, reactive, computed } from 'vue';
import GroupBox from '~/components/display/GroupBox.vue';
import Select from '~/components/input/Select.vue';
import Toggle from '~/components/input/Toggle.vue';
import Button from '~/components/base/Button.vue';

// 定义 props
const props = defineProps({
  selectedDataset: {
    type: Object,
    default: null,
  },
});

// 定义 emit 事件
const emit = defineEmits(['settings-changed']);

// 回放模式选项
const playbackModes = ref([
  {
    value: 'array',
    label: '阵元回放',
    description: '单个阵元数据回放',
    speed: '1倍速',
  },
  {
    value: 'config',
    label: '组态回放',
    description: '设备配置数据回放',
    speed: '1倍速',
  },
  {
    value: 'scenario',
    label: '场景回放',
    description: '综合场景数据回放',
    speed: '1-4倍速',
  },
]);

// 传输类型选项
const transportTypes = ref([
  { label: '广播 (Broadcast)', value: 'broadcast' },
  { label: '组播 (Multicast)', value: 'multicast' },
  { label: '单播 (Unicast)', value: 'unicast' },
]);

// 缓冲区大小选项
const bufferSizeOptions = ref([
  { label: '1 MB', value: 1048576 },
  { label: '2 MB', value: 2097152 },
  { label: '4 MB', value: 4194304 },
  { label: '8 MB', value: 8388608 },
  { label: '16 MB', value: 16777216 },
]);

// 回放设置
const playbackSettings = reactive({
  mode: 'scenario',
  transportType: 'broadcast',
  targetAddress: '192.168.1.255',
  targetPort: 12345,
  bufferSize: 4194304, // 4MB
  strictTiming: true,
  loopPlayback: false,
  autoUdpBroadcast: true,
});

// 计算总文件大小
const formatTotalSize = (files) => {
  const totalBytes = files.reduce((sum, file) => sum + file.size, 0);

  if (totalBytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(totalBytes) / Math.log(k));

  return parseFloat((totalBytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

// 应用设置
const applySettings = () => {
  console.log('应用回放设置:', playbackSettings);
  emit('settings-changed', {
    dataset: props.selectedDataset,
    settings: { ...playbackSettings },
  });
};

// 重置设置
const resetSettings = () => {
  playbackSettings.mode = 'scenario';
  playbackSettings.transportType = 'broadcast';
  playbackSettings.targetAddress = '192.168.1.255';
  playbackSettings.targetPort = 12345;
  playbackSettings.bufferSize = 4194304;
  playbackSettings.strictTiming = true;
  playbackSettings.loopPlayback = false;
  playbackSettings.autoUdpBroadcast = true;

  console.log('回放设置已重置');
};
</script>

<style scoped>
.settings-group {
  @apply mb-sm border-b border-border-divider pb-sm last:mb-0 last:border-b-0 last:pb-0;
}
</style>
