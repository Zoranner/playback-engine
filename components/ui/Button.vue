<template>
  <button 
    class="button" 
    :class="buttonClasses"
    :disabled="disabled"
    @click="handleClick"
  >
    <Icon v-if="icon && iconPosition === 'left'" :name="icon" class="button-icon button-icon--left" />
    <span v-if="$slots.default" class="button-text">
      <slot />
    </span>
    <Icon v-if="icon && iconPosition === 'right'" :name="icon" class="button-icon button-icon--right" />
  </button>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  // 按钮变体
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'primary', 'success', 'warning', 'danger', 'ghost', 'text'].includes(value)
  },
  
  // 按钮尺寸
  size: {
    type: String,
    default: 'medium',
    validator: (value) => ['small', 'medium', 'large'].includes(value)
  },
  
  // 图标
  icon: {
    type: String,
    default: ''
  },
  
  // 图标位置
  iconPosition: {
    type: String,
    default: 'left',
    validator: (value) => ['left', 'right'].includes(value)
  },
  
  // 状态
  disabled: {
    type: Boolean,
    default: false
  },
  
  // 方形按钮（通常用于只有图标的按钮）
  square: {
    type: Boolean,
    default: false
  },
  
  // 激活状态
  active: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['click']);

// 计算按钮样式类
const buttonClasses = computed(() => [
  `button--${props.variant}`,
  `button--${props.size}`,
  {
    'button--square': props.square,
    'button--active': props.active,
    'button--disabled': props.disabled,
    'button--icon-only': props.icon && !props.$slots?.default
  }
]);

// 处理点击事件
const handleClick = (event) => {
  if (!props.disabled) {
    emit('click', event);
  }
};
</script>

<style scoped>
.button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-xs);
  background: linear-gradient(145deg, var(--tertiary-bg), var(--secondary-bg));
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  font-family: var(--font-family-main);
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  user-select: none;
  position: relative;
  overflow: hidden;
}

/* 尺寸变体 */
.button--small {
  height: 24px;
  padding: 0 var(--spacing-sm);
  font-size: 11px;
}

.button--medium {
  height: 32px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
}

.button--large {
  height: 40px;
  padding: 0 var(--spacing-lg);
  font-size: 14px;
}

/* 方形按钮 */
.button--square.button--small {
  width: 24px;
  padding: 0;
}

.button--square.button--medium {
  width: 32px;
  padding: 0;
}

.button--square.button--large {
  width: 40px;
  padding: 0;
}

/* 颜色变体 */
.button--default:hover {
  background: linear-gradient(145deg, var(--border-color-light), var(--tertiary-bg));
  border-color: var(--border-color-light);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.button--primary {
  background: linear-gradient(145deg, #0ea5e9, #0284c7);
  border-color: #0ea5e9;
}

.button--primary:hover {
  background: linear-gradient(145deg, #38bdf8, #0ea5e9);
  box-shadow: var(--glow-subtle);
}

.button--success {
  background: linear-gradient(145deg, var(--success-color), #059669);
  border-color: var(--success-color);
}

.button--success:hover {
  background: linear-gradient(145deg, #34d399, var(--success-color));
  box-shadow: 0 0 8px rgba(16, 185, 129, 0.3);
}

.button--warning {
  background: linear-gradient(145deg, var(--warning-color), #d97706);
  border-color: var(--warning-color);
}

.button--warning:hover {
  background: linear-gradient(145deg, #fbbf24, var(--warning-color));
  box-shadow: 0 0 8px rgba(245, 158, 11, 0.3);
}

.button--danger {
  background: linear-gradient(145deg, var(--danger-color), #dc2626);
  border-color: var(--danger-color);
}

.button--danger:hover {
  background: linear-gradient(145deg, #f87171, var(--danger-color));
  box-shadow: 0 0 8px rgba(239, 68, 68, 0.3);
}

.button--ghost {
  background: transparent;
  border-color: var(--border-color);
}

.button--ghost:hover {
  background: var(--secondary-bg);
  border-color: var(--border-color-light);
}

.button--text {
  background: transparent;
  border: none;
  color: var(--text-accent);
}

.button--text:hover {
  background: rgba(56, 189, 248, 0.1);
  color: #38bdf8;
}

/* 状态 */
.button--active {
  background: var(--tertiary-bg);
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

.button--disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.button:active:not(.button--disabled) {
  transform: translateY(0);
  box-shadow: var(--shadow-inset);
}

/* 图标样式 */
.button-icon {
  width: 14px;
  height: 14px;
  flex-shrink: 0;
}

.button-icon--left {
  margin-right: calc(var(--spacing-xs) / 2);
}

.button-icon--right {
  margin-left: calc(var(--spacing-xs) / 2);
}

/* 只有图标的按钮 */
.button--icon-only .button-icon {
  margin: 0;
}

/* 悬停时的微妙动画效果 */
.button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.1), transparent);
  transition: left var(--transition-normal);
}

.button:hover::before:not(.button--disabled) {
  left: 100%;
}
</style>