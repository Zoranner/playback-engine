<template>
  <div :class="collapseClasses">
    <div v-for="(item, index) in items" :key="item.key || index" :class="itemClasses">
      <!-- 面板头部 -->
      <div :class="getHeaderClasses(item)" @click="handleHeaderClick(item.key || index, item)">
        <div class="flex min-w-0 flex-1 items-center">
          <!-- 标题 -->
          <span :class="getTitleClasses(item)">{{ item.label }}</span>
        </div>

        <!-- 展开图标 -->
        <div
          v-if="item.showArrow !== false && item.collapsible !== 'disabled'"
          :class="getArrowClasses(item, index)"
        >
          <slot name="expandIcon" :item="item" :expanded="activeKeys.includes(item.key || index)">
            <Icon name="heroicons:chevron-down" size="14" />
          </slot>
        </div>
      </div>

      <!-- 面板内容 -->
      <div
        v-if="activeKeys.includes(item.key || index) || item.forceRender"
        v-show="activeKeys.includes(item.key || index)"
        :class="contentClasses"
      >
        <div :class="bodyClasses">
          <slot :name="`content-${item.key || index}`" :item="item" :index="index">
            <component :is="item.children" v-if="item.children" />
          </slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

// Props
const props = defineProps({
  // 面板数据
  items: {
    type: Array,
    default: () => [],
  },
  // 当前激活的面板
  activeKey: {
    type: [String, Number, Array],
    default: undefined,
  },
  // 默认激活的面板
  defaultActiveKey: {
    type: [String, Number, Array],
    default: () => [],
  },
  // 手风琴模式
  accordion: {
    type: Boolean,
    default: false,
  },
  // 是否有边框
  bordered: {
    type: Boolean,
    default: true,
  },
  // 是否为简洁模式
  ghost: {
    type: Boolean,
    default: false,
  },
  // 尺寸
  size: {
    type: String,
    default: 'middle',
    validator: value => ['small', 'middle', 'large'].includes(value),
  },

  // 全局可折叠配置
  collapsible: {
    type: String,
    default: undefined,
    validator: value => [undefined, 'header', 'icon', 'disabled'].includes(value),
  },
});

// Events
const emit = defineEmits(['change', 'update:activeKey']);

// 内部状态
const activeKeys = ref([]);

// 初始化激活状态
const initActiveKeys = () => {
  if (props.activeKey !== undefined) {
    activeKeys.value = Array.isArray(props.activeKey) ? [...props.activeKey] : [props.activeKey];
  } else {
    activeKeys.value = Array.isArray(props.defaultActiveKey)
      ? [...props.defaultActiveKey]
      : [props.defaultActiveKey];
  }
};

// 主容器样式类
const collapseClasses = computed(() => {
  const baseClasses = ['overflow-hidden transition-all duration-normal'];

  if (props.ghost) {
    baseClasses.push('bg-transparent border-0 rounded-none');
  } else {
    baseClasses.push(
      'bg-background-panel border border-border rounded-md shadow-sm',
      'hover:border-border-light hover:shadow-md'
    );
  }

  return baseClasses;
});

// 项目样式类
const itemClasses = computed(() => [
  'relative overflow-hidden',
  !props.ghost && '[&:not(:last-child)]:border-b [&:not(:last-child)]:border-border-divider',
]);

// 头部样式类
const getHeaderClasses = item => {
  const baseClasses = ['flex items-center transition-all duration-fast relative'];

  // 尺寸样式
  const sizeClasses = {
    small: 'px-md py-xs',
    middle: 'px-md py-sm',
    large: 'px-lg py-md',
  };

  if (props.ghost) {
    baseClasses.push('bg-transparent');
    if (!item.disabled && item.collapsible !== 'disabled') {
      baseClasses.push('cursor-pointer hover:bg-background-secondary/50 rounded-sm');
    }
  } else {
    // 使用渐变背景，与Button组件类似
    baseClasses.push(
      'bg-gradient-to-r from-background-secondary to-background-tertiary',
      'border-b border-border-divider last:border-b-0'
    );

    if (!item.disabled && item.collapsible !== 'disabled') {
      baseClasses.push(
        'cursor-pointer',
        'hover:from-background-tertiary hover:to-background-secondary',
        'hover:shadow-glow-subtle',
        'active:from-background-secondary active:to-background-tertiary'
      );
    }
  }

  // 激活状态
  if (activeKeys.value.includes(item.key || (typeof index !== 'undefined' ? index : 0))) {
    if (!props.ghost) {
      baseClasses.push('from-primary/10 to-primary/5', 'border-primary/20 shadow-glow-subtle');
    }
  }

  return [...baseClasses, sizeClasses[props.size] || sizeClasses.middle];
};

// 标题样式类
const getTitleClasses = item => {
  const baseClasses = ['font-medium flex-1 min-w-0 transition-colors duration-fast'];

  if (item.disabled) {
    baseClasses.push('text-text-muted opacity-60');
  } else {
    baseClasses.push('text-text-primary');
    // 激活状态下的标题颜色
    if (activeKeys.value.includes(item.key || 0)) {
      baseClasses.push('text-text-accent');
    }
  }

  return baseClasses;
};

// 箭头样式类
const getArrowClasses = (item, index) => {
  const baseClasses = [
    'ml-md text-text-muted transition-all duration-fast',
    'flex items-center justify-center',
    'w-6 h-6 rounded-xs',
  ];

  if (activeKeys.value.includes(item.key || index)) {
    baseClasses.push('rotate-180 text-primary');
  }

  if (item.collapsible === 'icon') {
    baseClasses.push('cursor-pointer hover:bg-background-tertiary');
  }

  return baseClasses;
};

// 内容区域样式类
const contentClasses = computed(() => {
  const baseClasses = ['overflow-hidden transition-all duration-normal'];

  if (props.ghost) {
    baseClasses.push('bg-transparent');
  } else {
    baseClasses.push(
      'bg-gradient-to-br from-background-panel to-background-secondary',
      'border-t border-border-divider/50'
    );
  }

  return baseClasses;
});

// 内容体样式类
const bodyClasses = computed(() => {
  const baseClasses = ['transition-all duration-normal'];

  const sizeClasses = {
    small: 'p-sm',
    middle: 'p-md',
    large: 'p-lg',
  };

  return [...baseClasses, sizeClasses[props.size] || sizeClasses.middle];
});

// 监听 activeKey 变化
watch(
  () => props.activeKey,
  newVal => {
    if (newVal !== undefined) {
      activeKeys.value = Array.isArray(newVal) ? [...newVal] : [newVal];
    }
  },
  { immediate: true }
);

// 处理头部点击
const handleHeaderClick = (key, item) => {
  if (item.disabled || item.collapsible === 'disabled') return;

  const index = activeKeys.value.indexOf(key);
  const newActiveKeys = [...activeKeys.value];

  if (props.accordion) {
    // 手风琴模式
    if (index > -1) {
      newActiveKeys.splice(index, 1);
    } else {
      newActiveKeys.length = 0;
      newActiveKeys.push(key);
    }
  } else {
    // 普通模式
    if (index > -1) {
      newActiveKeys.splice(index, 1);
    } else {
      newActiveKeys.push(key);
    }
  }

  activeKeys.value = newActiveKeys;
  emit('update:activeKey', props.accordion ? newActiveKeys[0] : newActiveKeys);
  emit('change', props.accordion ? newActiveKeys[0] : newActiveKeys);
};

// 初始化
initActiveKeys();
</script>
