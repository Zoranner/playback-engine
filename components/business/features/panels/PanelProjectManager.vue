<template>
  <GroupBox
    title="工程管理"
    class="flex flex-1 flex-col overflow-hidden"
  >
    <!-- 数据集目录树 -->
    <div
      v-if="currentProject"
      class="flex flex-1 flex-col overflow-y-auto"
    >
      <!-- 工程项目列表 -->
      <div class="flex-1 overflow-y-auto">
        <!-- 工程根目录 -->
        <div
          :class="getProjectItemClasses(selectedItem === 'project')"
          @click="selectProject"
        >
          <Icon
            name="heroicons:folder"
            size="16"
            class="text-primary"
          />
          <span class="text-sm">{{ projectName }}</span>
        </div>

        <!-- 数据集列表 -->
        <div
          v-for="dataset in datasets"
          :key="dataset.name"
          :class="getDatasetItemClasses(selectedItem === dataset.name)"
          @click="selectDataset(dataset)"
        >
          <Icon
            name="heroicons:folder"
            size="16"
            class="text-warning"
          />
          <span class="text-sm">{{ dataset.name }}</span>
        </div>
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
import { invoke } from '@tauri-apps/api/core';
import { useProject } from '~/composables/useProject';
import GroupBox from '~/components/display/GroupBox.vue';

// 定义 emit 事件
const emit = defineEmits(['dataset-selected', 'project-selected']);

// 使用项目管理
const { currentProject, projectName } = useProject();

// 选中的项目（工程或数据集）
const selectedItem = ref(null);

// 添加调试日志
console.log('PanelProjectManager 初始化，当前项目:', currentProject.value);

// 模拟数据集数据（在实际项目中应该从后端获取）
const datasets = ref([]);

// 统一的项目样式类
const getProjectItemClasses = (isSelected) => {
  const baseClasses = [
    'flex items-center gap-2 px-3 py-2 cursor-pointer transition-colors',
    'hover:bg-background-tertiary/70'
  ];

  if (isSelected) {
    baseClasses.push('bg-primary/10 text-primary border-r-2 border-primary');
  } else {
    baseClasses.push('text-text-primary');
  }

  return baseClasses;
};

// 统一的数据集样式类（带缩进）
const getDatasetItemClasses = (isSelected) => {
  const baseClasses = [
    'flex items-center gap-2 px-3 py-2 ml-4 cursor-pointer transition-colors',
    'hover:bg-background-tertiary/70'
  ];

  if (isSelected) {
    baseClasses.push('bg-primary/10 text-primary border-r-2 border-primary');
  } else {
    baseClasses.push('text-text-secondary');
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

// 选择工程
const selectProject = () => {
  selectedItem.value = 'project';
  emit('project-selected', currentProject.value);
};

// 选择数据集
const selectDataset = (dataset) => {
  selectedItem.value = dataset.name;
  // 清除之前的选中状态
  datasets.value.forEach(d => d.selected = false);
  // 设置当前选中
  dataset.selected = true;
  // 触发事件
  emit('dataset-selected', dataset);
};

// 加载项目数据集
const loadProjectDatasets = async (projectPath) => {
  try {
    console.log('正在加载项目数据集:', projectPath);
    // 调用后端API获取数据集信息
    const projectStructure = await getProjectStructure(projectPath);
    datasets.value = projectStructure.datasets.map(dataset => ({
      name: dataset.name,
      files: dataset.files.map(file => ({
        name: file.name,
        type: file.type || getFileTypeFromName(file.name),
        size: file.size,
        path: file.path
      }))
    }));
    console.log('数据集加载完成:', datasets.value);

    // 默认选中工程
    selectedItem.value = 'project';
  } catch (error) {
    console.error('加载项目数据集失败:', error);
  }
};

// 监听项目变化，加载数据集信息
watch(currentProject, async (newProject) => {
  console.log('PanelProjectManager watch 触发，新项目:', newProject);
  if (newProject) {
    console.log('项目已更新，正在加载数据集信息:', newProject);
    await loadProjectDatasets(newProject.path);
  } else {
    console.log('项目已清空，清除数据集列表');
    // 清除数据集列表
    datasets.value = [];
    selectedItem.value = null;
  }
}, { immediate: true });

// 获取项目结构（调用后端API）
const getProjectStructure = async (path) => {
  try {
    console.log('调用后端 get_project_structure API，路径:', path);
    const result = await invoke('get_project_structure', { projectPath: path });
    console.log('后端返回的项目结构:', result);
    return {
      datasets: result.datasets.map(dataset => ({
        name: dataset.name,
        files: dataset.files.map(file => ({
          name: file.name,
          size: file.size,
          path: file.path,
          type: file.type
        }))
      }))
    };
  } catch (error) {
    console.error('获取项目结构失败:', error);
    throw error;
  }
};

// 从文件名获取文件类型（作为后备方案）
const getFileTypeFromName = (filename) => {
  const ext = filename.toLowerCase().split('.').pop();
  switch (ext) {
    case 'pcap':
      return 'pcap';
    case 'pidx':
      return 'pidx';
    default:
      return 'unknown';
  }
};

// 获取文件类型（保持向后兼容）
const getFileType = (filename) => {
  return getFileTypeFromName(filename);
};
</script>
