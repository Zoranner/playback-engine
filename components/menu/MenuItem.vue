<template>
  <div
    class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast"
    :class="[
      disabled
        ? 'pointer-events-none cursor-not-allowed opacity-50'
        : 'hover:bg-background-tertiary',
      variant === 'danger' ? 'text-danger hover:bg-danger hover:text-background-primary' : '',
    ]"
    @click="handleClick"
  >
    <!-- 图标 -->
    <Icon v-if="icon" :name="icon" class="shrink-0" />

    <!-- 文字内容 -->
    <span class="flex-1">
      <slot>{{ label }}</slot>
    </span>

    <!-- 快捷键 -->
    <span v-if="shortcut" class="ml-auto text-text-secondary opacity-60">
      {{ shortcut }}
    </span>
  </div>
</template>

<script setup lang="ts">
interface Props {
  label?: string;
  icon?: string;
  shortcut?: string;
  disabled?: boolean;
  variant?: 'default' | 'danger';
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
  icon: '',
  shortcut: '',
  disabled: false,
  variant: 'default',
});

const emit = defineEmits<{
  click: [event: MouseEvent];
}>();

const handleClick = (event: MouseEvent) => {
  if (!props.disabled) {
    emit('click', event);
  }
};
</script>
