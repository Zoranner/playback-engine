<template>
  <aside class="flex w-panel flex-col gap-sm">
    <PanelProjectManager
      class="h-full flex-1"
      @dataset-selected="handleDatasetSelected"
    />
    <PanelPlaybackSettings
      class="flex-none"
      :selected-dataset="selectedDataset"
      @settings-changed="handleSettingsChanged"
    />
  </aside>
</template>

<script setup>
import { ref } from 'vue';
import PanelProjectManager from './PanelProjectManager.vue';
import PanelPlaybackSettings from './PanelPlaybackSettings.vue';

// 选中的数据集
const selectedDataset = ref(null);

// 处理数据集选择
const handleDatasetSelected = (dataset) => {
  selectedDataset.value = dataset;
  console.log('数据集已选择:', dataset.name);
};

// 处理设置变更
const handleSettingsChanged = (data) => {
  console.log('回放设置已变更:', data);
  // 这里可以调用后端API保存设置或触发回放配置更新
};
</script>
