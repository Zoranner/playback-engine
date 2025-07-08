<template>
  <GroupBox title="环境信息" class="environment-info-container">
    <div class="env-data">
      <div v-for="item in environmentData" :key="item.label" class="env-item">
        <span class="env-label text-caption">{{ item.label }}</span>
        <span class="env-value text-monospace">{{ item.value }}</span>
      </div>
    </div>
  </GroupBox>
</template>

<script setup>
import { useEnvironment } from '~/composables/useEnvironment';
import GroupBox from '~/components/ui/GroupBox.vue';

const { environmentData } = useEnvironment();
</script>

<style scoped>
.environment-info-container {
  flex: 0 0 auto;
}

.env-data {
  margin: calc(-1 * var(--spacing-sm));
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.env-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--tertiary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
}

.env-item:hover {
  background: var(--panel-bg);
  border-color: var(--border-color-light);
}

.env-label {
  font-size: 10px;
}

.env-value {
  font-size: 10px;
  font-weight: 600;
}
</style> 