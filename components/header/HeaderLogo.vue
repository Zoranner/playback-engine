<template>
  <div class="logo-section">
    <div class="logo-icon">
      <Icon name="heroicons:globe-asia-australia" size="28" />
    </div>
    <div class="title-container">
      <span class="system-title text-title">综合复盘软件</span>
      <span class="project-name text-caption">{{ currentProject || '未打开工程' }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
// 当前工程名称，后续可以通过props或store管理
const currentProject = ref('南海演习回放数据_20241223')
</script>

<style scoped>
.logo-section {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.logo-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  color: var(--text-accent);
  box-shadow: var(--shadow-sm);
}

.title-container {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.system-title {
  font-size: 16px;
  font-weight: 600;
  line-height: 1.2;
}

.project-name {
  font-size: 10px;
  opacity: 0.8;
  line-height: 1.2;
}
</style> 