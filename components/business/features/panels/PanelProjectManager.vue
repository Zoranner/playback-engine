<template>
  <GroupBox
    title="工程管理"
    class="flex flex-1 flex-col overflow-hidden"
  >
    <!-- 数据集目录树 -->
    <div
      v-if="currentProject"
      class="flex flex-1 flex-col overflow-y-auto p-xs"
    >
      <!-- 工程项目列表 -->
      <div class="flex-1 overflow-y-auto">
        <!-- 工程根目录 -->
        <div
          :class="getItemClasses(selectedItem === 'project', false)"
          @click="selectProject"
        >
          <Icon
            name="heroicons:folder"
            size="16"
            class="text-primary"
          />
          <span class="text-sm font-medium">{{ projectName }}</span>
          <Button
            class="ml-auto"
            variant="ghost"
            size="small"
            icon="heroicons:plus"
            @click.stop="showCreateDatasetDialog = true"
          />
        </div>

        <!-- 数据集列表和文件列表 -->
        <template
          v-for="dataset in datasets"
          :key="dataset.name"
        >
          <div
            :class="getItemClasses(selectedItem === dataset.name, true)"
            @click="selectDataset(dataset)"
          >
            <Icon
              name="heroicons:folder"
              size="16"
              class="text-warning"
            />
            <span class="text-sm">{{ dataset.name }}</span>
            <span
              v-if="dataset.files && dataset.files.length > 0"
              class="ml-auto text-xs text-text-muted"
            >
              {{ dataset.files.length }} 个文件
            </span>
          </div>

          <!-- 数据集文件列表 -->
          <div
            v-if="selectedItem === dataset.name"
            :key="`${dataset.name}-files`"
            class="ml-6 mt-xs flex flex-col gap-xs"
          >
            <div
              v-for="file in dataset.files"
              :key="file.path"
              :class="getFileItemClasses(file)"
              @click.stop="selectFile(file)"
            >
              <Icon
                :name="getFileIcon(file.type)"
                :class="getFileIconClass(file.type)"
                size="14"
              />
              <div class="flex-1 truncate text-xs">
                <span class="font-mono">{{ file.name }}</span>
                <span class="ml-xs text-text-muted">({{ formatFileSize(file.size) }})</span>
              </div>
            </div>
          </div>
        </template>
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

    <!-- 创建数据集对话框 -->
    <Modal
      v-model="showCreateDatasetDialog"
      title="创建新数据集"
      confirm-text="确认创建"
      cancel-text="取消"
      @confirm="createDataset"
    >
      <div class="mb-md">
        <label class="mb-xs block text-sm font-medium text-text-secondary"> 数据集名称 </label>
        <input
          v-model="newDatasetName"
          type="text"
          class="w-full rounded-sm border border-border bg-background-secondary px-sm py-xs text-text-primary transition-colors duration-fast focus:border-border-active focus:outline-none focus:ring-1 focus:ring-primary"
          placeholder="请输入数据集名称"
          @keyup.enter="createDataset"
        />
        <div
          v-if="datasetNameError"
          class="mt-xs text-sm text-danger"
        >
          {{ datasetNameError }}
        </div>
      </div>
    </Modal>
  </GroupBox>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useProject } from '~/composables/useProject';
import GroupBox from '~/components/display/GroupBox.vue';
import Button from '~/components/base/Button.vue';
import Modal from '~/components/base/modal/Modal.vue';

// 定义 emit 事件
const emit = defineEmits(['dataset-selected', 'project-selected', 'file-selected']);

// 使用工程管理
const { currentProject, projectName } = useProject();

// 选中的项目（工程或数据集）
const selectedItem = ref(null);

// 创建数据集相关状态
const showCreateDatasetDialog = ref(false);
const newDatasetName = ref('');
const datasetNameError = ref('');

// 添加调试日志
console.log('PanelProjectManager 初始化，当前项目:', currentProject.value);

// 模拟数据集数据（在实际项目中应该从后端获取）
const datasets = ref([]);

// 统一的项目样式类
const getItemClasses = (isSelected, isDataset = false) => {
  const baseClasses = [
    'flex items-center gap-sm px-sm py-xs rounded-sm transition-all duration-fast relative',
    'cursor-pointer select-none',
    isDataset ? 'ml-4' : '',
  ];

  if (isSelected) {
    baseClasses.push(
      'bg-background-panel border border-border-active shadow-glow',
      'text-text-primary font-medium'
    );
  } else {
    baseClasses.push(
      'bg-transparent border border-transparent',
      'text-text-primary hover:bg-background-secondary hover:border-border hover:shadow-sm'
    );
  }

  return baseClasses;
};

// 文件项样式类
const getFileItemClasses = file => {
  const baseClasses = [
    'flex items-center gap-sm px-sm py-xs rounded-sm transition-all duration-fast relative',
    'cursor-pointer select-none text-text-secondary',
    'hover:bg-background-secondary hover:shadow-sm',
  ];

  // 如果文件被选中，添加选中样式
  if (file.selected) {
    baseClasses.push('bg-background-panel border border-border-active shadow-glow-subtle');
  }

  return baseClasses;
};

// 获取文件图标
const getFileIcon = fileType => {
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
const getFileIconClass = fileType => {
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
const formatFileSize = bytes => {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
};

// 选择工程
const selectProject = () => {
  selectedItem.value = 'project';
  emit('project-selected', currentProject.value);
};

// 选择数据集
const selectDataset = dataset => {
  // 如果点击的是已选中的数据集，则取消选中（折叠文件列表）
  if (selectedItem.value === dataset.name) {
    selectedItem.value = null;
  } else {
    selectedItem.value = dataset.name;
    // 触发事件
    emit('dataset-selected', dataset);
  }
};

// 选择文件
const selectFile = file => {
  // 清除之前选中的文件状态
  datasets.value.forEach(dataset => {
    if (dataset.files) {
      dataset.files.forEach(f => (f.selected = false));
    }
  });

  // 设置当前文件为选中状态
  file.selected = true;

  // 触发事件
  emit('file-selected', file);
};

// 验证数据集名称
const validateDatasetName = () => {
  datasetNameError.value = '';

  if (!newDatasetName.value.trim()) {
    datasetNameError.value = '数据集名称不能为空';
    return false;
  }

  if (newDatasetName.value.length > 50) {
    datasetNameError.value = '数据集名称不能超过50个字符';
    return false;
  }

  // 检查是否已存在同名数据集
  if (datasets.value.some(dataset => dataset.name === newDatasetName.value)) {
    datasetNameError.value = '已存在同名数据集';
    return false;
  }

  return true;
};

// 创建数据集
const createDataset = async () => {
  if (!validateDatasetName()) {
    return;
  }

  try {
    // 调用后端API创建数据集
    const result = await invoke('create_dataset', {
      projectPath: currentProject.value.path,
      datasetName: newDatasetName.value,
    });

    if (result.success) {
      // 添加新数据集到列表
      datasets.value.push({
        name: newDatasetName.value,
        files: [],
      });

      // 关闭对话框并重置表单
      showCreateDatasetDialog.value = false;
      newDatasetName.value = '';
      datasetNameError.value = '';

      // 显示成功消息（如果项目中有消息提示组件的话）
      console.log('数据集创建成功');
    } else {
      datasetNameError.value = result.error || '创建数据集失败';
    }
  } catch (error) {
    console.error('创建数据集失败:', error);
    datasetNameError.value = '创建数据集时发生错误';
  }
};

// 加载项目数据集
const loadProjectDatasets = async projectPath => {
  try {
    console.log('正在加载项目数据集:', projectPath);
    // 调用后端API获取数据集信息
    const projectStructure = await getProjectStructure(projectPath);
    datasets.value = projectStructure.datasets.map(dataset => ({
      name: dataset.name,
      files: dataset.files.map(file => ({
        name: file.name,
        type: file.type ?? getFileTypeFromName(file.name),
        size: file.size,
        path: file.path,
      })),
    }));
    console.log('数据集加载完成:', datasets.value);

    // 默认选中工程
    selectedItem.value = 'project';
  } catch (error) {
    console.error('加载项目数据集失败:', error);
  }
};

// 监听项目变化，加载数据集信息
watch(
  currentProject,
  async newProject => {
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
  },
  { immediate: true }
);

// 获取项目结构（调用后端API）
const getProjectStructure = async path => {
  try {
    console.log('调用后端 get_project_structure API，路径:', path);
    const result = await invoke('get_project_structure', {
      projectPath: path,
    });
    console.log('后端返回的项目结构:', result);
    return {
      datasets: result.datasets.map(dataset => ({
        name: dataset.name,
        files: dataset.files.map(file => ({
          name: file.name,
          size: file.size,
          path: file.path,
          type: file.type,
        })),
      })),
    };
  } catch (error) {
    console.error('获取项目结构失败:', error);
    throw error;
  }
};

// 从文件名获取文件类型（作为后备方案）
const getFileTypeFromName = filename => {
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
const getFileType = filename => {
  return getFileTypeFromName(filename);
};
</script>
