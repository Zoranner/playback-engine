<template>
  <div :class="cardClasses">
    <!-- 头部 -->
    <div
      v-if="title || $slots.header"
      class="flex items-center justify-between border-b border-border px-md py-sm"
    >
      <slot name="header">
        <h3 class="m-0 font-semibold text-text-primary">{{ title }}</h3>
      </slot>
      <div
        v-if="$slots.actions"
        class="flex items-center gap-sm"
      >
        <slot name="actions" />
      </div>
    </div>

    <!-- 内容 -->
    <div :class="contentClasses">
      <slot />
    </div>

    <!-- 底部 -->
    <div
      v-if="$slots.footer"
      class="flex items-center justify-between border-t border-border-divider bg-background-secondary px-lg py-sm"
    >
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  title: {
    type: String,
    default: '',
  },
  active: {
    type: Boolean,
    default: false,
  },
  hoverable: {
    type: Boolean,
    default: true,
  },
  padded: {
    type: Boolean,
    default: true,
  },
  variant: {
    type: String,
    default: 'default',
    validator: value => ['default', 'ghost', 'elevated'].includes(value),
  },
});

// 卡片容器样式类
const cardClasses = computed(() => {
  const baseClasses = [
    'relative overflow-hidden transition-all duration-normal',
    'border rounded-md',
  ];

  // 变体样式
  const variantClasses = {
    default: [
      'bg-background-panel border-border shadow-sm',
      props.hoverable && 'hover:border-border-light hover:shadow-md',
      props.active && 'border-border-active shadow-glow',
    ],
    ghost: ['bg-transparent border-transparent shadow-none'],
    elevated: [
      'bg-background-panel border-border shadow-lg',
      props.hoverable && 'hover:border-border-light hover:shadow-xl',
      props.active && 'border-border-active shadow-glow',
    ],
  };

  return [
    ...baseClasses,
    ...(variantClasses[props.variant] || variantClasses.default).filter(Boolean),
  ];
});

// 内容区域样式类
const contentClasses = computed(() => ['relative', props.padded && 'p-md']);
</script>
