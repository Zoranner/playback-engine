<template>
  <button
    :class="buttonClasses"
    :disabled="disabled"
    @click="handleClick"
  >
    <Icon
      v-if="icon && iconPosition === 'left'"
      :name="icon"
      :class="iconClasses"
    />
    <span
      v-if="$slots.default"
      class="truncate"
    >
      <slot />
    </span>
    <Icon
      v-if="icon && iconPosition === 'right'"
      :name="icon"
      :class="iconClasses"
    />
  </button>
</template>

<script setup>
import { computed, provide } from 'vue';

const props = defineProps({
  // 按钮变体
  variant: {
    type: String,
    default: 'default',
    validator: value =>
      ['default', 'primary', 'success', 'warning', 'danger', 'ghost', 'text'].includes(value),
  },

  // 按钮尺寸
  size: {
    type: String,
    default: 'default',
    validator: value => ['small', 'default', 'large'].includes(value),
  },

  // 图标
  icon: {
    type: String,
    default: '',
  },

  // 图标位置
  iconPosition: {
    type: String,
    default: 'left',
    validator: value => ['left', 'right'].includes(value),
  },

  // 状态
  disabled: {
    type: Boolean,
    default: false,
  },

  // 方形按钮（通常用于只有图标的按钮）
  square: {
    type: Boolean,
    default: false,
  },

  // 激活状态
  active: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['click']);

// 图标尺寸映射
const iconSizeMap = {
  small: 'h-3 w-3',
  default: 'h-4 w-4',
  large: 'h-5 w-5',
};

// 计算图标样式类
const iconClasses = computed(() => [iconSizeMap[props.size], 'shrink-0']);

// 向子组件提供尺寸信息
provide('buttonSize', props.size);
provide('iconClasses', iconClasses.value);

// 计算按钮样式类
const buttonClasses = computed(() => {
  // 尺寸样式
  const sizeClasses = {
    small: ['h-6 px-xs text-sm', props.square && 'w-6 px-0'],
    default: ['h-8 px-sm', props.square && 'w-8 px-0'],
    large: ['h-10 px-lg text-lg', props.square && 'w-10 px-0'],
  };

  const baseClasses = [
    // 基础样式
    'inline-flex items-center justify-center gap-xs',
    'font-medium cursor-pointer select-none',
    'border rounded-sm transition-all duration-fast',
    'relative overflow-hidden',
    // 尺寸样式
    ...sizeClasses[props.size].filter(Boolean),
    // 禁用状态
    props.disabled && 'opacity-50 cursor-not-allowed',
    // 激活状态
    props.active && 'bg-background-tertiary border-border-active shadow-glow',
  ];

  // 变体样式
  const variantClasses = {
    default: [
      'bg-gradient-to-br from-background-tertiary to-background-secondary',
      'border-border text-text-primary',
      'hover:from-border-light hover:to-background-tertiary hover:border-border-light',
      'hover:-translate-y-px hover:shadow-sm',
      'active:translate-y-0 active:shadow-inset',
    ],
    primary: [
      'bg-gradient-to-br from-primary to-primary-600',
      'border-primary text-white',
      'hover:from-primary-400 hover:to-primary',
      'hover:shadow-glow',
      'active:translate-y-0',
    ],
    success: [
      'bg-gradient-to-br from-success to-success-dark',
      'border-success text-white',
      'hover:from-success-light hover:to-success',
      'hover:shadow-glow-strong',
      'active:translate-y-0',
    ],
    warning: [
      'bg-gradient-to-br from-warning to-warning-dark',
      'border-warning text-white',
      'hover:from-warning-light hover:to-warning',
      'active:translate-y-0',
    ],
    danger: [
      'bg-gradient-to-br from-danger to-danger-dark',
      'border-danger text-white',
      'hover:from-danger-light hover:to-danger',
      'active:translate-y-0',
    ],
    ghost: [
      'bg-transparent border-border text-text-primary',
      'hover:bg-background-secondary hover:border-border-light',
      'active:translate-y-0',
    ],
    text: [
      'bg-transparent border-transparent text-text-accent',
      'hover:bg-primary/10 hover:text-primary-light',
      'active:translate-y-0',
    ],
  };

  return [
    ...baseClasses.filter(Boolean),
    ...(variantClasses[props.variant] || variantClasses.default),
  ];
});

// 处理点击事件
const handleClick = event => {
  if (!props.disabled) {
    emit('click', event);
  }
};
</script>
