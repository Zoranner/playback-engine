<template>
  <div ref="dropdownRef" class="relative">
    <!-- 触发器 -->
    <div
      class="flex cursor-pointer select-none items-center gap-xs whitespace-nowrap rounded-xs bg-transparent px-sm py-xs transition-all duration-fast hover:bg-background-tertiary hover:shadow-glow-subtle"
      :class="{ 'bg-background-tertiary shadow-glow-subtle': isOpen }"
      @click="toggle"
    >
      <slot name="trigger" :is-open="isOpen">
        <span class="font-medium text-text-primary">{{ label }}</span>
        <Icon
          name="heroicons:chevron-down"
          class="text-text-secondary transition-transform duration-fast"
          :class="{ 'rotate-180': isOpen }"
        />
      </slot>
    </div>

    <!-- 下拉内容 -->
    <div
      v-if="isOpen"
      class="absolute z-dropdown overflow-hidden rounded-md border border-border bg-background-secondary shadow-md backdrop-blur-md"
      :class="[positionClasses, 'min-w-[180px]']"
    >
      <slot name="content" :close="close" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';

interface Props {
  label?: string;
  position?: 'left' | 'right';
  minWidth?: string;
  modelValue?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  label: '',
  position: 'right',
  minWidth: '180px',
  modelValue: false,
});

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  open: [];
  close: [];
}>();

const isOpen = ref(props.modelValue);
const dropdownRef = ref<HTMLElement | null>(null);

const positionClasses = computed(() => {
  return props.position === 'left' ? 'left-0 top-[calc(100%+4px)]' : 'right-0 top-[calc(100%+4px)]';
});

const toggle = () => {
  isOpen.value = !isOpen.value;
  emit('update:modelValue', isOpen.value);

  if (isOpen.value) {
    emit('open');
  } else {
    emit('close');
  }
};

const close = () => {
  isOpen.value = false;
  emit('update:modelValue', false);
  emit('close');
};

// 点击外部区域关闭菜单
const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    if (isOpen.value) {
      close();
    }
  }
};

// 监听modelValue变化
watch(
  () => props.modelValue,
  newValue => {
    isOpen.value = newValue;
  }
);

// 监听isOpen变化，管理全局点击事件
watch(isOpen, newValue => {
  if (newValue) {
    // 延迟添加事件监听器，避免立即触发
    setTimeout(() => {
      document.addEventListener('click', handleClickOutside);
    }, 0);
  } else {
    document.removeEventListener('click', handleClickOutside);
  }
});

// 组件卸载时清理事件监听器
onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});
</script>
