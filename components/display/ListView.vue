<template>
  <div :class="containerClasses">
    <!-- 头部区域 -->
    <div
      v-if="title || $slots.header || searchable"
      class="flex items-center justify-between border-b border-border p-sm"
    >
      <div class="flex items-center gap-sm">
        <h3 v-if="title" class="m-0 font-semibold text-text-primary">{{ title }}</h3>
        <slot name="header" />
      </div>

      <div class="flex items-center gap-sm">
        <!-- 搜索框 -->
        <input
          v-if="searchable"
          v-model="searchQuery"
          type="text"
          :placeholder="searchPlaceholder"
          class="rounded-sm border border-border bg-background-secondary px-sm py-xs text-text-primary placeholder-text-muted focus:border-border-active focus:outline-none"
        />

        <slot name="actions" />
      </div>
    </div>

    <!-- 列表内容 -->
    <div :class="contentClasses">
      <!-- 空状态 -->
      <div
        v-if="filteredItems.length === 0"
        class="flex flex-col items-center justify-center py-lg text-text-muted"
      >
        <Icon v-if="emptyIcon" :name="emptyIcon" class="mb-sm h-8 w-8 opacity-50" />
        <div class="text-center">
          <div class="mb-xs font-medium">{{ emptyTitle }}</div>
          <div class="text-sm">{{ emptyDescription }}</div>
        </div>
      </div>

      <!-- 列表项 -->
      <template v-else>
        <div
          v-for="(item, index) in filteredItems"
          :key="getItemKey(item, index)"
          :class="itemWrapperClasses"
          @click="() => handleItemClick(item)"
        >
          <slot
            :item="item"
            :index="index"
            :selected="isSelected(item)"
            :select="() => selectItem(item)"
            :toggle="() => toggleItem(item)"
          >
            <!-- 默认的简单列表项渲染 -->
            <div
              class="flex cursor-pointer items-start gap-sm rounded-sm border border-border bg-background-tertiary p-xs transition-all duration-fast hover:bg-background-panel"
              :class="{ 'border-border-active bg-background-panel shadow-glow': isSelected(item) }"
            >
              <div class="flex-1 leading-normal">
                {{ getItemLabel(item) }}
              </div>
            </div>
          </slot>
        </div>
      </template>
    </div>

    <!-- 底部区域 -->
    <div
      v-if="$slots.footer || showSelection"
      class="flex items-center justify-between border-t border-border bg-background-secondary p-sm"
    >
      <div v-if="showSelection" class="text-sm text-text-muted">
        已选择 {{ selectedItems.length }} / {{ filteredItems.length }} 项
      </div>
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  // 数据
  items: {
    type: Array,
    default: () => [],
  },

  // 标题
  title: {
    type: String,
    default: '',
  },

  // 选择模式
  selectionMode: {
    type: String,
    default: 'none',
    validator: value => ['none', 'single', 'multiple'].includes(value),
  },

  // 已选择的项
  selectedItems: {
    type: Array,
    default: () => [],
  },

  // 搜索功能
  searchable: {
    type: Boolean,
    default: false,
  },
  searchPlaceholder: {
    type: String,
    default: '搜索...',
  },
  searchFields: {
    type: Array,
    default: () => ['label', 'title', 'name'],
  },

  // 数据获取函数
  itemKey: {
    type: [String, Function],
    default: 'id',
  },
  itemLabel: {
    type: [String, Function],
    default: 'label',
  },

  // 空状态
  emptyTitle: {
    type: String,
    default: '暂无数据',
  },
  emptyDescription: {
    type: String,
    default: '当前列表为空',
  },
  emptyIcon: {
    type: String,
    default: 'inbox',
  },

  // 样式
  variant: {
    type: String,
    default: 'default',
    validator: value => ['default', 'compact', 'bordered'].includes(value),
  },

  // 显示选择状态
  showSelection: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['update:selectedItems', 'select', 'click', 'search']);

// 搜索查询
const searchQuery = ref('');

// 容器样式类
const containerClasses = computed(() => {
  const baseClasses = ['bg-background-panel border border-border rounded-md overflow-hidden'];

  if (props.variant === 'bordered') {
    baseClasses.push('border-2');
  }

  return baseClasses;
});

// 内容区域样式类
const contentClasses = computed(() => {
  const baseClasses = ['overflow-y-auto'];

  if (props.variant === 'compact') {
    baseClasses.push('max-h-60');
  } else {
    baseClasses.push('max-h-96');
  }

  return baseClasses;
});

// 列表项包装器样式类
const itemWrapperClasses = computed(() => {
  const baseClasses = [];

  if (props.variant === 'compact') {
    baseClasses.push('p-1');
  } else {
    baseClasses.push('p-xs');
  }

  return baseClasses;
});

// 过滤后的列表项
const filteredItems = computed(() => {
  if (!searchQuery.value || !props.searchable) {
    return props.items;
  }

  const query = searchQuery.value.toLowerCase();

  return props.items.filter(item => {
    return props.searchFields.some(field => {
      const value = typeof item === 'object' ? item[field] : item;
      return String(value || '')
        .toLowerCase()
        .includes(query);
    });
  });
});

// 获取项目键值
const getItemKey = (item, index) => {
  if (typeof props.itemKey === 'function') {
    return props.itemKey(item, index);
  }
  if (typeof item === 'object' && item[props.itemKey]) {
    return item[props.itemKey];
  }
  return index;
};

// 获取项目标签
const getItemLabel = item => {
  if (typeof props.itemLabel === 'function') {
    return props.itemLabel(item);
  }
  if (typeof item === 'object' && item[props.itemLabel]) {
    return item[props.itemLabel];
  }
  return String(item);
};

// 检查项目是否被选中
const isSelected = item => {
  if (props.selectionMode === 'none') return false;

  const key = getItemKey(item);
  return props.selectedItems.some(selected => getItemKey(selected) === key);
};

// 选择项目（替换选择）
const selectItem = item => {
  if (props.selectionMode === 'none') return;

  let newSelection = [];

  if (props.selectionMode === 'single') {
    newSelection = [item];
  } else if (props.selectionMode === 'multiple') {
    newSelection = [item];
  }

  emit('update:selectedItems', newSelection);
  emit('select', item);
};

// 切换项目选择状态
const toggleItem = item => {
  if (props.selectionMode === 'none') return;

  const isCurrentlySelected = isSelected(item);
  let newSelection = [...props.selectedItems];

  if (props.selectionMode === 'single') {
    newSelection = isCurrentlySelected ? [] : [item];
  } else if (props.selectionMode === 'multiple') {
    if (isCurrentlySelected) {
      const key = getItemKey(item);
      newSelection = newSelection.filter(selected => getItemKey(selected) !== key);
    } else {
      newSelection.push(item);
    }
  }

  emit('update:selectedItems', newSelection);
  emit('select', item);
};

// 处理项目点击
const handleItemClick = item => {
  emit('click', item);

  if (props.selectionMode !== 'none') {
    toggleItem(item);
  }
};

// 监听搜索查询变化
watch(searchQuery, newQuery => {
  emit('search', newQuery);
});
</script>
