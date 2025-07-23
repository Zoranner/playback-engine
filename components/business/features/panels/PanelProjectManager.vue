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
      <TreeView
        :items="treeData"
        :selected-items="selectedItems"
        :expanded-items="expandedItems"
        selection-mode="single"
        item-key="id"
        item-label="name"
        item-children="children"
        :item-icon="getIcon"
        :item-icon-class="getIconClass"
        @select="handleSelect"
        @expand="handleExpand"
        @collapse="handleCollapse"
        @create-dataset="showCreateDatasetDialog = true"
      />
    </div>

    <!-- 无工程时的提示 -->
    <EmptyPlaceholder
      v-else
      icon="heroicons:folder-open"
      title="未打开工程"
      description="请先打开回放工程"
      icon-size="32"
    />

    <!-- 创建数据集对话框 -->
    <Modal
      :visible="showCreateDatasetDialog"
      title="创建新数据集"
      confirm-text="确认创建"
      cancel-text="取消"
      @update:visible="showCreateDatasetDialog = $event"
      @confirm="createDataset"
    >
      <Input
        v-model="newDatasetName"
        label="数据集名称"
        placeholder="请输入数据集名称"
        :error="datasetNameError"
        @enter="createDataset"
      />
    </Modal>
  </GroupBox>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useProject } from '~/composables/useProject';
import GroupBox from '~/components/display/GroupBox.vue';
import Button from '~/components/base/Button.vue';
import Modal from '~/components/base/Modal.vue';
import Input from '~/components/base/Input.vue';
import TreeView from '~/components/display/TreeView.vue';
import EmptyPlaceholder from '~/components/display/EmptyPlaceholder.vue';

// 定义 emit 事件
const emit = defineEmits(['dataset-selected', 'project-selected', 'file-selected']);

// 使用工程管理
const { currentProject, projectName } = useProject();

// 选中的项目
const selectedItems = ref([]);

// 展开的项目
const expandedItems = ref([]);

// 树状数据
const treeData = computed(() => {
  if (!currentProject.value) return [];

  // 构建工程项
  const projectItem = {
    id: 'project',
    name: projectName.value,
    type: 'project',
    children: datasets.value.map(dataset => ({
      id: `dataset-${dataset.name}`,
      name: dataset.name,
      type: 'dataset',
      dataset,
      children:
        dataset.files?.map(file => ({
          id: `file-${file.path}`,
          name: file.name,
          type: 'file',
          file,
          size: file.size,
          fileType: file.type,
        })) ?? [],
    })),
  };

  return [projectItem];
});

// 创建数据集相关状态
const showCreateDatasetDialog = ref(false);
const newDatasetName = ref('');
const datasetNameError = ref('');

// 添加调试日志
console.log('PanelProjectManager 初始化，当前项目:', currentProject.value);

// 模拟数据集数据（在实际项目中应该从后端获取）
const datasets = ref([]);

// 处理选择事件
const handleSelect = item => {
  // 更新选中项状态
  selectedItems.value = [item];
  
  // 清除之前选中的文件状态
  datasets.value.forEach(dataset => {
    if (dataset.files) {
      dataset.files.forEach(f => (f.selected = false));
    }
  });

  // 根据项目类型触发相应的事件
  if (item.type === 'project') {
    emit('project-selected', currentProject.value);
  } else if (item.type === 'dataset') {
    emit('dataset-selected', item.dataset);
  } else if (item.type === 'file') {
    // 设置当前文件为选中状态
    item.file.selected = true;
    emit('file-selected', item.file);
  }
};

// 处理展开事件
const handleExpand = item => {
  // 将展开的项目添加到expandedItems数组中
  expandedItems.value = [...expandedItems.value, item];
};

// 处理收起事件
const handleCollapse = item => {
  // 从expandedItems数组中移除收起的项目
  const itemId = item.id;
  expandedItems.value = expandedItems.value.filter(expanded => expanded.id !== itemId);
};

// 获取项目图标
const getIcon = item => {
  switch (item.type) {
    case 'project':
      return 'heroicons:folder';
    case 'dataset':
      return 'heroicons:folder';
    case 'file':
      switch (item.fileType) {
        case 'pcap':
          return 'heroicons:document-duplicate';
        case 'pidx':
          return 'heroicons:document-magnifying-glass';
        default:
          return 'heroicons:document';
      }
    default:
      return 'heroicons:document';
  }
};

// 获取图标样式类
const getIconClass = item => {
  switch (item.type) {
    case 'project':
      return 'text-primary';
    case 'dataset':
      return 'text-warning';
    case 'file':
      switch (item.fileType) {
        case 'pcap':
          return 'text-success';
        case 'pidx':
          return 'text-info';
        default:
          return 'text-text-muted';
      }
    default:
      return 'text-text-muted';
  }
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

      // 展开工程项以显示新创建的数据集
      // 由于treeData是计算属性，添加数据集后会自动更新
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

    // 不默认选中任何项，保持所有项样式一致
    selectedItems.value = [];
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
      selectedItems.value = [];
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

// 格式化文件大小
const formatFileSize = bytes => {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
};
</script>
