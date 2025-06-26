<template>
  <div 
    class="military-card" 
    :class="{ 
      'military-card--active': active,
      'military-card--hover': hoverable 
    }"
  >
    <div v-if="title" class="card-header">
      <h3 class="card-title">{{ title }}</h3>
      <div v-if="$slots.actions" class="card-actions">
        <slot name="actions" />
      </div>
    </div>
    <div class="card-content" :class="{ 'card-content--padded': padded }">
      <slot />
    </div>
  </div>
</template>

<script setup>
defineProps({
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
  }
})
</script>

<style scoped>
.military-card {
  background: var(--panel-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition-normal);
  overflow: hidden;
}

.military-card--hover:hover {
  border-color: var(--border-color-light);
  box-shadow: var(--shadow-md);
}

.military-card--active {
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-md) var(--spacing-lg);
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

.card-content--padded {
  padding: var(--spacing-lg);
}
</style> 