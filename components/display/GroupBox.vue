<template>
  <div :class="groupboxClasses">
    <!-- 顶部装饰线 -->
    <div class="absolute left-0 right-0 top-0 h-px bg-gradient-to-r from-border-light to-border" />

    <div
      class="flex items-center justify-between border-b border-border-divider bg-background-header px-sm py-xs"
    >
      <h4 class="m-0 text-xs font-semibold uppercase tracking-wide text-text-secondary">
        {{ title }}
      </h4>
      <div
        v-if="$slots.actions"
        class="flex items-center gap-xs"
      >
        <slot name="actions" />
      </div>
    </div>

    <div :class="contentClasses">
      <slot />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  padded: {
    type: Boolean,
    default: true,
  },
  variant: {
    type: String,
    default: 'default',
    validator: value => ['default', 'flat', 'raised'].includes(value),
  },
});

// 分组框样式类
const groupboxClasses = computed(() => {
  const baseClasses = ['border border-border rounded-md relative overflow-hidden'];

  // 变体样式
  const variantClasses = {
    default: ['bg-gradient-to-b from-background-secondary to-background-tertiary', 'shadow-inner'],
    flat: ['bg-background-secondary shadow-none'],
    raised: ['bg-gradient-to-b from-background-secondary to-background-tertiary', 'shadow-md'],
  };

  return [...baseClasses, ...(variantClasses[props.variant] ?? variantClasses.default)];
});

// 内容区域样式类
const contentClasses = computed(() => ['relative', props.padded && 'p-sm']);
</script>
