<template>
  <div 
    class="list-item"
    :class="itemClass"
    @click="handleClick"
    :tabindex="selectable ? 0 : -1"
    @keydown.enter="handleClick"
  >
    <!-- 指示器 -->
    <div 
      v-if="indicator"
      class="list-indicator"
      :class="indicatorClass"
      :style="indicatorStyle"
    />
    
    <!-- 图标 -->
    <Icon 
      v-if="icon" 
      :name="icon" 
      :size="iconSize" 
      class="list-icon"
      :style="{ color: iconColor }"
    />
    
    <!-- 内容区域 -->
    <div class="list-content">
      <!-- 主要内容 -->
      <div class="list-main">
        <slot />
      </div>
      
      <!-- 元数据 -->
      <div v-if="$slots.meta || meta" class="list-meta">
        <slot name="meta">
          <span class="text-monospace">{{ meta }}</span>
        </slot>
      </div>
    </div>
    
    <!-- 操作区域 -->
    <div v-if="$slots.actions" class="list-actions">
      <slot name="actions" />
    </div>
    
    <!-- 状态指示器 -->
    <div 
      v-if="status"
      class="list-status"
      :class="`list-status--${status}`"
    />
  </div>
</template>

<script setup>
import { computed } from 'vue'


const props = defineProps({
  // 基础属性
  active: {
    type: Boolean,
    default: false
  },
  selectable: {
    type: Boolean,
    default: true
  },
  disabled: {
    type: Boolean,
    default: false
  },
  
  // 视觉样式
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'compact', 'comfortable'].includes(value)
  },
  
  // 指示器
  indicator: {
    type: Boolean,
    default: false
  },
  indicatorColor: {
    type: String,
    default: 'var(--border-color)'
  },
  indicatorType: {
    type: String,
    default: 'line',
    validator: (value) => ['line', 'dot', 'square'].includes(value)
  },
  
  // 图标
  icon: {
    type: String,
    default: ''
  },
  iconColor: {
    type: String,
    default: 'currentColor'
  },
  iconSize: {
    type: [String, Number],
    default: 16
  },
  
  // 状态
  status: {
    type: String,
    default: '',
    validator: (value) => ['', 'online', 'warning', 'offline', 'success', 'danger'].includes(value)
  },
  
  // 元数据
  meta: {
    type: String,
    default: ''
  },
  
  // 分组类型（用于特定样式）
  type: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'target', 'event', 'env', 'legend', 'menu'].includes(value)
  }
})

const emit = defineEmits(['click', 'select'])

const itemClass = computed(() => [
  `list-item--${props.variant}`,
  `list-item--${props.type}`,
  {
    'list-item--active': props.active,
    'list-item--disabled': props.disabled,
    'list-item--selectable': props.selectable && !props.disabled
  }
])

const indicatorClass = computed(() => [
  `list-indicator--${props.indicatorType}`,
  { 'list-indicator--active': props.active }
])

const indicatorStyle = computed(() => {
  const styles = {}
  
  if (props.indicatorType === 'line') {
    styles.borderLeftColor = props.indicatorColor
  } else {
    styles.backgroundColor = props.indicatorColor
  }
  
  return styles
})

const handleClick = (event) => {
  if (props.disabled) return
  
  emit('click', event)
  if (props.selectable) {
    emit('select', event)
  }
}
</script>

<style scoped>
.list-item {
  display: flex;
  align-items: flex-start;
  background: var(--tertiary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
  position: relative;
  overflow: hidden;
}

.list-item--selectable {
  cursor: pointer;
}

.list-item--selectable:hover:not(.list-item--disabled) {
  background: var(--panel-bg);
  border-color: var(--border-color-light);
  transform: translateX(2px);
}

.list-item--active {
  background: var(--panel-bg);
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

.list-item--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 变体样式 */
.list-item--compact {
  padding: var(--spacing-xs) var(--spacing-sm);
  gap: var(--spacing-xs);
}

.list-item--default {
  padding: var(--spacing-xs) var(--spacing-sm);
  gap: var(--spacing-sm);
}

.list-item--comfortable {
  padding: var(--spacing-sm) var(--spacing-md);
  gap: var(--spacing-md);
}

/* 特定类型样式 */
.list-item--target {
  border-left: 3px solid var(--border-color);
}

.list-item--event {
  border-left: 3px solid var(--border-color);
}

.list-item--legend {
  padding: 1px 2px;
  border: none;
  background: transparent;
}

.list-item--legend:hover {
  background: var(--border-color);
  transform: none;
}

.list-item--menu {
  border: none;
  border-radius: var(--radius-xs);
  background: transparent;
}

.list-item--menu:hover {
  background: var(--tertiary-bg);
  transform: none;
}

/* 指示器 */
.list-indicator {
  flex-shrink: 0;
  transition: all var(--transition-fast);
}

.list-indicator--line {
  width: 3px;
  height: 100%;
  position: absolute;
  left: 0;
  top: 0;
  border-left: 3px solid;
}

.list-indicator--dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-top: 2px;
  box-shadow: 0 0 4px currentColor;
}

.list-indicator--square {
  width: 8px;
  height: 8px;
  border-radius: 1px;
  margin-top: 2px;
}

/* 图标 */
.list-icon {
  flex-shrink: 0;
  margin-top: 2px;
}

/* 内容区域 */
.list-content {
  flex: 1;
  min-width: 0;
}

.list-main {
  font-size: 11px;
  line-height: 1.4;
}

.list-meta {
  margin-top: 2px;
  font-size: 9px;
  color: var(--text-muted);
}

/* 操作区域 */
.list-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  flex-shrink: 0;
}

/* 状态指示器 */
.list-status {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
  margin-top: 2px;
  box-shadow: 0 0 4px currentColor;
}

.list-status--online {
  background: var(--success-color);
  color: var(--success-color);
}

.list-status--warning {
  background: var(--warning-color);
  color: var(--warning-color);
}

.list-status--offline {
  background: var(--danger-color);
  color: var(--danger-color);
}

.list-status--success {
  background: var(--success-color);
  color: var(--success-color);
}

.list-status--danger {
  background: var(--danger-color);
  color: var(--danger-color);
}

/* 焦点样式 */
.list-item:focus-visible {
  outline: 2px solid var(--border-color-active);
  outline-offset: 2px;
}
</style> 