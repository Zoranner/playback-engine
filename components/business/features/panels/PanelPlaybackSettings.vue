<template>
  <GroupBox
    title="回放设置"
    class="flex-none"
  >
    <!-- 固定高度的设置区域 -->
    <div class="-m-sm flex flex-col min-h-fit max-h-60 overflow-y-auto gap-xs p-sm">
      <!-- 有工程且选中数据集时显示设置 -->
      <template v-if="currentProject && selectedDataset">
        <!-- 传输类型 -->
        <div>
          <div class="mb-2 text-sm font-medium text-text-primary">传输类型</div>
          <Select
            v-model="playbackSettings.transportType"
            :options="transportTypes"
            placeholder="选择传输类型"
            size="small"
          />
        </div>

        <!-- 目标地址 -->
        <div>
          <div class="mb-2 text-sm font-medium text-text-primary">目标地址</div>
          <input
            v-model="playbackSettings.targetAddress"
            type="text"
            class="w-full rounded-sm border border-border bg-background-primary px-3 py-2 text-sm transition-colors focus:border-border-active focus:outline-none"
            placeholder="192.168.1.255"
          />
        </div>

        <!-- 目标端口 -->
        <div>
          <div class="mb-2 text-sm font-medium text-text-primary">目标端口</div>
          <input
            v-model="playbackSettings.targetPort"
            type="number"
            min="1"
            max="65535"
            class="w-full rounded-sm border border-border bg-background-primary px-3 py-2 text-sm transition-colors focus:border-border-active focus:outline-none"
            placeholder="12345"
          />
        </div>
      </template>

      <!-- 占位内容 -->
      <div
        v-else
        class="flex flex-1 flex-col items-center justify-center gap-sm py-lg text-center text-text-muted min-h-64"
      >
        <Icon
          name="heroicons:cog-6-tooth"
          size="24"
          class="opacity-60"
        />
        <div class="text-text-secondary">{{ getPlaceholderTitle() }}</div>
        <div class="text-caption opacity-70">{{ getPlaceholderDescription() }}</div>
      </div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref, reactive, watch, computed } from 'vue';
import Select from '~/components/input/Select.vue';
import GroupBox from '~/components/display/GroupBox.vue';
import { useProject } from '~/composables/useProject';

// Props
const props = defineProps({
  selectedDataset: {
    type: Object,
    default: null
  }
});

// Events
const emit = defineEmits(['settings-changed']);

// 获取当前工程状态
const { currentProject } = useProject();

// 传输类型选项
const transportTypes = [
  { label: 'UDP广播', value: 'udp_broadcast' },
  { label: 'UDP单播', value: 'udp_unicast' },
  { label: 'TCP', value: 'tcp' }
];

// 回放设置
const playbackSettings = reactive({
  transportType: 'udp_broadcast',
  targetAddress: '192.168.1.255',
  targetPort: 12345
});

// 获取占位内容标题
const getPlaceholderTitle = () => {
  if (!currentProject.value) {
    return '未打开工程';
  }
  if (!props.selectedDataset) {
    return '未选择数据集';
  }
  return '等待选择';
};

// 获取占位内容描述
const getPlaceholderDescription = () => {
  if (!currentProject.value) {
    return '请先打开回放工程';
  }
  if (!props.selectedDataset) {
    return '请在工程管理中选择数据集';
  }
  return '请选择要回放的内容';
};

// 监听设置变化
watch(
  () => playbackSettings,
  (newSettings) => {
    if (currentProject.value && props.selectedDataset) {
      emit('settings-changed', {
        dataset: props.selectedDataset?.name || null,
        settings: { ...newSettings }
      });
    }
  },
  { deep: true }
);
</script>
