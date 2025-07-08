<template>
  <div 
    class="card" 
    :class="cardClass"
  >
    <div v-if="title || $slots.header" class="card-header">
      <slot name="header">
        <h3 class="card-title">{{ title }}</h3>
      </slot>
      <div v-if="$slots.actions" class="card-actions">
        <slot name="actions" />
      </div>
    </div>
    <div class="card-content" :class="{ 'card-content--padded': padded }">
      <slot />
    </div>
    <div v-if="$slots.footer" class="card-footer">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  title: {
    type: String,
    default: ''
  },
  active: {
    type: Boolean,
    default: false
  },
  hoverable: {
    type: Boolean,
    default: true
  },
  padded: {
    type: Boolean,
    default: true
  },
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'ghost', 'elevated'].includes(value)
  },
  size: {
    type: String,
    default: 'medium',
    validator: (value) => ['small', 'medium', 'large'].includes(value)
  }
});

const cardClass = computed(() => [
  `card--${props.variant}`,
  `card--${props.size}`,
  {
    'card--active': props.active,
    'card--hoverable': props.hoverable
  }
]);
</script>

<style scoped>
.card {
  background: var(--panel-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-normal);
  overflow: hidden;
  position: relative;
}

.card--hoverable:hover {
  border-color: var(--border-color-light);
  box-shadow: var(--shadow-md);
}

.card--active {
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

/* 变体样式 */
.card--ghost {
  background: transparent;
  border-color: transparent;
  box-shadow: none;
}

.card--elevated {
  box-shadow: var(--shadow-lg);
}

/* 尺寸变体 */
.card--small .card-header {
  padding: var(--spacing-sm) var(--spacing-md);
}

.card--small .card-content--padded {
  padding: var(--spacing-md);
}

.card--medium .card-header {
  padding: var(--spacing-md) var(--spacing-lg);
}

.card--medium .card-content--padded {
  padding: var(--spacing-lg);
}

.card--large .card-header {
  padding: var(--spacing-lg) var(--spacing-xl);
}

.card--large .card-content--padded {
  padding: var(--spacing-xl);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border-color);
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
}

.card-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.card-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.card-content {
  position: relative;
}

.card-footer {
  padding: var(--spacing-sm) var(--spacing-lg);
  border-top: 1px solid var(--divider-color);
  background: var(--secondary-bg);
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style> 