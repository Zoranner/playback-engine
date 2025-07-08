<template>
  <div class="groupbox" :class="groupboxClass">
    <div class="groupbox-header">
      <h4 class="groupbox-title">{{ title }}</h4>
      <div v-if="$slots.actions" class="groupbox-actions">
        <slot name="actions" />
      </div>
    </div>
    <div class="groupbox-content" :class="{ 'groupbox-content--padded': padded }">
      <slot />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  title: {
    type: String,
    required: true
  },
  padded: {
    type: Boolean,
    default: true
  },
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'flat', 'raised'].includes(value)
  },
  size: {
    type: String,
    default: 'medium',
    validator: (value) => ['small', 'medium', 'large'].includes(value)
  }
})

const groupboxClass = computed(() => [
  `groupbox--${props.variant}`,
  `groupbox--${props.size}`
])
</script>

<style scoped>
.groupbox {
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-inset);
  position: relative;
  overflow: hidden;
}

.groupbox::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-color-light), transparent);
}

/* 变体样式 */
.groupbox--flat {
  background: var(--secondary-bg);
  box-shadow: none;
}

.groupbox--raised {
  box-shadow: var(--shadow-md);
}

.groupbox-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--divider-color);
  background: rgba(255, 255, 255, 0.02);
}

/* 尺寸变体 */
.groupbox--small .groupbox-header {
  padding: var(--spacing-xs) var(--spacing-sm);
}

.groupbox--small .groupbox-content--padded {
  padding: var(--spacing-sm);
}

.groupbox--medium .groupbox-header {
  padding: var(--spacing-xs) var(--spacing-sm);
}

.groupbox--medium .groupbox-content--padded {
  padding: var(--spacing-sm);
}

.groupbox--large .groupbox-header {
  padding: var(--spacing-sm) var(--spacing-md);
}

.groupbox--large .groupbox-content--padded {
  padding: var(--spacing-md);
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
}
</style> 