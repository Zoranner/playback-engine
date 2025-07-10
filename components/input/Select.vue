<template>
  <div :class="containerClasses">
    <label v-if="label" class="mb-xs block font-medium text-text-secondary">
      {{ label }}
    </label>

    <div class="relative">
      <div
        :class="triggerClasses"
        :tabindex="disabled ? -1 : 0"
        @click="toggleOpen"
        @keydown.enter.space.prevent="toggleOpen"
        @keydown.esc="closeSelect"
      >
        <span class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-left">
          {{ displayValue }}
        </span>
        <Icon name="heroicons:chevron-down" :class="arrowClasses" />
      </div>

      <!-- 移除 Transition，直接显示 -->
      <div v-if="isOpen" ref="dropdownRef" :class="dropdownClasses">
        <div
          v-for="option in options"
          :key="option.value"
          :class="getOptionClasses(option)"
          @click="selectOption(option)"
        >
          <Icon
            v-if="option.icon"
            :name="option.icon"
            class="h-3.5 w-3.5 flex-shrink-0 text-text-secondary"
          />
          <span class="flex-1">{{ option.label }}</span>
          <Icon
            v-if="option.value === modelValue"
            name="heroicons:check"
            class="h-3 w-3 flex-shrink-0 text-text-accent"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';

const props = defineProps({
  modelValue: {
    type: [String, Number, Boolean],
    required: true,
  },
  options: {
    type: Array,
    required: true,
    validator: options => {
      return options.every(
        option => typeof option === 'object' && 'value' in option && 'label' in option
      );
    },
  },
  label: {
    type: String,
    default: '',
  },
  placeholder: {
    type: String,
    default: '请选择',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
  variant: {
    type: String,
    default: 'default',
    validator: value => ['default', 'ghost'].includes(value),
  },
});

const emit = defineEmits(['update:modelValue', 'change']);

const isOpen = ref(false);
const dropdownRef = ref(null);

// 容器样式类
const containerClasses = computed(() => ['relative w-full', props.disabled && 'opacity-50']);

// 触发器样式类
const triggerClasses = computed(() => {
  const baseClasses = [
    'flex items-center justify-between h-8 px-md border rounded-sm',
    'text-text-primary cursor-pointer transition-all duration-fast select-none relative',
  ];

  if (props.disabled) {
    baseClasses.push('opacity-50 cursor-not-allowed');
  } else {
    baseClasses.push('hover:border-border-light');

    // 变体样式
    if (props.variant === 'ghost') {
      baseClasses.push(
        'bg-transparent border-transparent hover:bg-background-secondary hover:border-border'
      );
    } else {
      baseClasses.push('bg-background-secondary border-border hover:bg-background-tertiary');
    }
  }

  if (isOpen.value) {
    baseClasses.push('border-border-active shadow-glow');
  }

  return baseClasses;
});

// 箭头样式类
const arrowClasses = computed(() => [
  'w-3 h-3 text-text-secondary transition-transform duration-fast flex-shrink-0 ml-xs',
  isOpen.value && 'rotate-180',
]);

// 下拉菜单样式类
const dropdownClasses = computed(() => [
  'absolute top-full left-0 right-0 mt-1 bg-background-secondary border border-border',
  'rounded-md shadow-md z-50 max-h-50 overflow-y-auto backdrop-blur-sm',
]);

// 获取选项样式类
const getOptionClasses = option => {
  const baseClasses = [
    'flex items-center gap-sm px-md py-sm cursor-pointer transition-colors duration-fast',
  ];

  if (option.disabled) {
    baseClasses.push('opacity-50 cursor-not-allowed');
  } else {
    baseClasses.push('hover:bg-background-tertiary');
  }

  if (option.value === props.modelValue) {
    baseClasses.push('bg-border text-text-accent');
  }

  return baseClasses;
};

const displayValue = computed(() => {
  const selected = props.options.find(option => option.value === props.modelValue);
  return selected?.label || props.placeholder;
});

const toggleOpen = () => {
  if (props.disabled) return;
  isOpen.value = !isOpen.value;
};

const closeSelect = () => {
  isOpen.value = false;
};

const selectOption = option => {
  if (option.disabled) return;

  emit('update:modelValue', option.value);
  emit('change', option);
  closeSelect();
};

const handleClickOutside = event => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target)) {
    closeSelect();
  }
};

onMounted(() => {
  document.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
});

watch(isOpen, newValue => {
  if (newValue) {
    document.addEventListener('click', handleClickOutside);
  }
});
</script>
