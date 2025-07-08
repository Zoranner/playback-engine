<template>
  <GroupBox title="目标信息" class="target-info-container">
    <div class="target-list scrollbar">
      <div 
        v-for="target in targets" 
        :key="target.id" 
        class="target-item" 
        :class="{ 'target-item--active': selectedTarget === target.id }"
        @click="handleSelectTarget(target)"
      >
        <div class="target-indicator" :style="{ background: target.color }"/>
        <div class="target-details">
          <div class="target-name text-subtitle">{{ target.label }}</div>
          <div class="target-id text-monospace">{{ target.id }}</div>
          <div class="target-info-row">
            <span class="target-distance text-caption">距离: </span>
            <span class="text-monospace">{{ target.distance }}海里</span>
          </div>
          <div class="target-info-row">
            <span class="target-bearing text-caption">方位: </span>
            <span class="text-monospace">{{ target.bearing }}°</span>
          </div>
        </div>
        <div class="target-status" :class="`target-status--${target.status}`"/>
      </div>
    </div>
  </GroupBox>
</template>

<script setup>
import { ref } from 'vue'
import { useTargets } from '~/composables/useTargets'
import GroupBox from '~/components/ui/GroupBox.vue'

const { targets, selectTarget } = useTargets()
const selectedTarget = ref(null)

const handleSelectTarget = (target) => {
  selectedTarget.value = selectedTarget.value === target.id ? null : target.id
  selectTarget(target)
}
</script>

<style scoped>
.target-info-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.target-list {
  flex: 1;
  margin: calc(-1 * var(--spacing-sm));
  padding: var(--spacing-sm);
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  overflow-y: auto;
}

.target-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm);
  background: var(--tertiary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.target-item:hover {
  background: var(--panel-bg);
  border-color: var(--border-color-light);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.target-item--active {
  background: var(--panel-bg);
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

.target-indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  flex-shrink: 0;
  box-shadow: 0 0 4px currentColor;
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.target-details {
  flex: 1;
  min-width: 0;
}

.target-name {
  font-size: 12px;
  margin-bottom: 2px;
  font-weight: 600;
}

.target-id {
  font-size: 9px;
  margin-bottom: 2px;
  opacity: 0.8;
}

.target-info-row {
  display: flex;
  align-items: center;
  margin-bottom: 1px;
  font-size: 10px;
}

.target-distance, 
.target-bearing {
  min-width: 30px;
}

.target-status {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
  box-shadow: 0 0 3px currentColor;
}

.target-status--friendly { 
  background: var(--success-color);
  color: var(--success-color);
}

.target-status--neutral { 
  background: var(--warning-color);
  color: var(--warning-color);
}

.target-status--unknown { 
  background: var(--danger-color);
  color: var(--danger-color);
}
</style> 