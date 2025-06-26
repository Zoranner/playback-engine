<template>
  <div class="military-groupbox">
    <div class="groupbox-header">
      <h4 class="groupbox-title">{{ title }}</h4>
      <div v-if="$slots.actions" class="groupbox-actions">
        <slot name="actions" />
      </div>
    </div>
    <div class="groupbox-content">
      <slot />
    </div>
  </div>
</template>

<script setup>
defineProps({
  title: {
    type: String,
    required: true
  }
})
</script>

<style scoped>
.military-groupbox {
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-inset);
  position: relative;
  overflow: hidden;
}

.military-groupbox::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-color-light), transparent);
}

.groupbox-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-xs) var(--spacing-sm);
  border-bottom: 1px solid var(--divider-color);
  background: rgba(255, 255, 255, 0.02);
}

.groupbox-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  margin: 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.groupbox-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}

.groupbox-content {
  position: relative;
  padding: var(--spacing-sm);
}
</style> 