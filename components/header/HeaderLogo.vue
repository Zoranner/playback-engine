<template>
  <div class="logo-section">
    <div class="logo-icon">
      <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <circle cx="12" cy="12" r="3"/>
        <path d="M12 1v6M12 17v6M4.22 4.22l4.24 4.24M15.54 15.54l4.24 4.24M1 12h6M17 12h6M4.22 19.78l4.24-4.24M15.54 8.46l4.24-4.24"/>
      </svg>
    </div>
    <div class="title-container">
      <span class="system-title text-title">海域信息态势感知系统</span>
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