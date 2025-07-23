<template>
  <div :class="placeholderClasses">
    <Icon
      v-if="icon"
      :name="icon"
      :size="iconSize"
      :class="iconClasses"
    />
    <div
      v-if="title"
      class="text-text-secondary"
      :class="titleClasses"
    >
      {{ title }}
    </div>
    <div
      v-if="description"
      class="text-caption opacity-70"
      :class="descriptionClasses"
    >
      {{ description }}
    </div>
    <slot />
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  icon: {
    type: String,
    default: '',
  },
  title: {
    type: String,
    default: '',
  },
  description: {
    type: String,
    default: '',
  },
  size: {
    type: String,
    default: 'md',
    validator: value => ['sm', 'md', 'lg'].includes(value),
  },
  iconSize: {
    type: String,
    default: '24',
  },
  vertical: {
    type: Boolean,
    default: true,
  },
});

// 占位符容器样式类
const placeholderClasses = computed(() => {
  const baseClasses = [
    'flex flex-col items-center justify-center text-center text-text-muted',
    'gap-sm py-lg',
  ];

  // 尺寸变体
  const sizeClasses = {
    sm: ['gap-xs py-md'],
    md: ['gap-sm py-lg'],
    lg: ['gap-md py-xl'],
  };

  // 方向变体
  const directionClasses = props.vertical ? ['flex-col'] : ['flex-row'];

  return [...baseClasses, ...(sizeClasses[props.size] ?? sizeClasses.md), ...directionClasses];
});

// 图标样式类
const iconClasses = computed(() => {
  return ['opacity-60'];
});

// 标题样式类
const titleClasses = computed(() => {
  const sizeClasses = {
    sm: ['text-sm'],
    md: ['text-base'],
    lg: ['text-lg'],
  };

  return [...(sizeClasses[props.size] ?? sizeClasses.md)];
});

// 描述样式类
const descriptionClasses = computed(() => {
  const sizeClasses = {
    sm: ['text-xs'],
    md: ['text-sm'],
    lg: ['text-base'],
  };

  return [...(sizeClasses[props.size] ?? sizeClasses.md)];
});
</script>
