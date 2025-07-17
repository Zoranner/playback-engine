<template>
  <GroupBox
    title="工程管理"
    class="flex flex-1 flex-col overflow-hidden"
  >
    <!-- 工程信息 -->
    <div
      v-if="currentProject"
      class="mb-sm border-b border-border-divider pb-sm"
    >
      <div class="mb-1 font-semibold text-text-primary">{{ currentProject.name }}</div>
      <div class="text-caption text-text-secondary">
        文件数量: {{ currentProject.fileCount }} |
        时长: {{ formattedDuration }}
      </div>
    </div>

    <!-- 数据集目录树 -->
    <div
      v-if="currentProject"
      class="-m-sm flex flex-1 flex-col gap-xs overflow-y-auto p-sm"
    >
      <!-- 工程根目录 -->
      <div class="flex items-center gap-sm text-text-primary">
        <Icon
          name="heroicons:folder"
          size="16"
          class="text-text-secondary"
        />
        <span class="text-subtitle font-medium">{{ projectName }}</span>
      </div>

      <!-- 数据集列表 -->
      <div
        v-for="dataset in datasets"
        :key="dataset.name"
        class="ml-4"
      >
        <div
          :class="getDatasetItemClasses(dataset)"
          @click="selectDataset(dataset)"
        >
          <Icon
            :name="dataset.expanded ? 'heroicons:folder-open' : 'heroicons:folder'"
            size="16"
            class="text-warning"
          />
          <span class="flex-1 text-subtitle">{{ dataset.name }}</span>
          <span class="text-caption text-text-muted">{{ dataset.files.length }} 文件</span>
          <Icon
            :name="dataset.expanded ? 'heroicons:chevron-down' : 'heroicons:chevron-right'"
            size="12"
            class="text-text-muted"
          />
        </div>

        <!-- 数据集文件列表 -->
        <div
          v-if="dataset.expanded"
          class="ml-6 mt-xs"
        >
          <div
            v-for="file in dataset.files"
            :key="file.name"
            class="flex items-center gap-sm py-xs px-sm rounded-sm hover:bg-background-tertiary/50 transition-colors"
          >
            <Icon
              :name="getFileIcon(file.type)"
              size="14"
              :class="getFileIconClass(file.type)"
            />
            <span class="flex-1 text-caption font-mono">{{ file.name }}</span>
            <span class="text-caption text-text-muted">{{ formatFileSize(file.size) }}</span>
          </div>
        </div>
      </div>

      <!-- 工程文件 -->
      <div class="ml-4 flex items-center gap-sm py-xs">
        <Icon
          name="heroicons:document-text"
          size="16"
          class="text-primary"
        />
        <span class="text-caption font-mono">project.pproj</span>
      </div>
    </div>

    <!-- 无工程时的提示 -->
    <div
      v-else
      class="flex flex-1 flex-col items-center justify-center gap-sm py-lg text-center text-text-muted"
    >
      <Icon
        name="heroicons:folder-open"
        size="32"
        class="opacity-60"
      />
      <div class="text-text-secondary">未打开工程</div>
      <div class="text-caption opacity-70">请先打开回放工程</div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { useProject } from '~/composables/useProject';
import GroupBox from '~/components/display/GroupBox.vue';

// 定义 emit 事件
const emit = defineEmits(['dataset-selected']);

// 使用项目管理
const { currentProject, projectName, formattedDuration } = useProject();

// 选中的数据集
const selectedDataset = ref(null);

// 模拟数据集数据（在实际项目中应该从后端获取）
const datasets = ref([
  {
    name: 'data-set-01',
    expanded: false,
    selected: false,
    files: [
      {
        name: 'data_240321_153045_1234567.pcap',
        type: 'pcap',
        size: 134217728, // 128MB
      },
      {
        name: 'data_240321_154012_4567890.pcap',
        type: 'pcap',
        size: 268435456, // 256MB
      },
      {
        name: 'data_240321_155130_7890123.pcap',
        type: 'pcap',
        size: 104857600, // 100MB
      },
      {
        name: 'data-set-01.pidx',
        type: 'pidx',
        size: 1048576, // 1MB
      },
    ],
  },
  {
    name: 'data-set-02',
    expanded: false,
    selected: false,
    files: [
      {
        name: 'data_240321_153045_1234567.pcap',
        type: 'pcap',
        size: 209715200, // 200MB
      },
      {
        name: 'data_240321_154012_4567890.pcap',
        type: 'pcap',
        size: 314572800, // 300MB
      },
      {
        name: 'data_240321_155130_7890123.pcap',
        type: 'pcap',
        size: 157286400, // 150MB
      },
      {
        name: 'data-set-02.pidx',
        type: 'pidx',
        size: 1572864, // 1.5MB
      },
    ],
  },
]);

// 数据集项样式类
const getDatasetItemClasses = (dataset) => {
  const baseClasses = [
    'flex items-center gap-sm p-sm bg-background-tertiary border border-border rounded-sm',
    'cursor-pointer transition-all duration-fast relative',
  ];

  if (dataset.selected) {
    baseClasses.push('bg-background-panel border-border-active shadow-glow');
  } else {
    baseClasses.push(
      'hover:bg-background-panel hover:border-border-light hover:-translate-y-px hover:shadow-sm'
    );
  }

  return baseClasses;
};

// 获取文件图标
const getFileIcon = (fileType) => {
  switch (fileType) {
    case 'pcap':
      return 'heroicons:document-duplicate';
    case 'pidx':
      return 'heroicons:document-magnifying-glass';
    default:
      return 'heroicons:document';
  }
};

// 获取文件图标样式类
const getFileIconClass = (fileType) => {
  switch (fileType) {
    case 'pcap':
      return 'text-success';
    case 'pidx':
      return 'text-info';
    default:
      return 'text-text-muted';
  }
};

// 格式化文件大小
const formatFileSize = (bytes) => {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};

// 选择数据集
const selectDataset = (dataset) => {
  // 清除之前的选中状态
  datasets.value.forEach(d => d.selected = false);

  // 设置当前选中
  dataset.selected = true;
  dataset.expanded = !dataset.expanded;
  selectedDataset.value = dataset;

  // 触发事件
  emit('dataset-selected', dataset);
};

// 监听项目变化，重置数据集列表
watch(currentProject, (newProject) => {
  if (newProject) {
    // 在实际项目中，这里应该从后端获取数据集信息
    console.log('项目已更新，应从后端获取数据集信息:', newProject);
  } else {
    // 清除选中状态
    datasets.value.forEach(d => {
      d.selected = false;
      d.expanded = false;
    });
    selectedDataset.value = null;
  }
});
</script>
