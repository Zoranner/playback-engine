<template>
  <div>
    <!-- 项目项 -->
    <div
      :class="itemWrapperClasses"
      @click="handleClick"
    >
      <slot
        :item="item"
        :level="level"
        :selected="isSelected"
        :expanded="isExpanded"
        :toggle="toggle"
        :select="select"
      >
        <!-- 默认的树状项渲染 -->
        <div :class="itemClasses">
          <!-- 缩进 -->
          <div
            v-for="i in level"
            :key="i"
            class="w-xs"
          />

          <!-- 展开/收起图标 -->
          <Icon
            v-if="hasChildren"
            :name="isExpanded ? 'heroicons:chevron-down' : 'heroicons:chevron-right'"
            class="h-4 w-4 text-text-muted"
            @click.stop="toggle"
          />

          <!-- 占位符（无子项时） -->
          <div
            v-else
            class="w-4"
          />

          <!-- 项目图标 -->
          <Icon
            v-if="iconName"
            :name="iconName"
            :class="iconClass"
            class="h-4 w-4"
          />

          <!-- 项目标签 -->
          <div class="flex-1 truncate text-sm leading-normal">
            {{ label }}
            <span
              v-if="item.size !== undefined"
              class="ml-xs text-text-muted"
            >
              ({{ formatFileSize(item.size) }})
            </span>
          </div>

          <!-- 新建数据集按钮（仅在工程项上显示） -->
          <Button
            v-if="item.type === 'project'"
            class="ml-auto"
            variant="ghost"
            size="small"
            icon="heroicons:plus"
            @click.stop="$emit('create-dataset', item)"
          />

          <!-- 子项数量 -->
          <span
            v-if="childrenCount > 0"
            class="ml-auto text-xs text-text-muted"
          >
            {{ childrenCount }} 项
          </span>
        </div>
      </slot>
    </div>

    <!-- 子项 -->
    <div
      v-if="isExpanded && hasChildren"
      class="ml-4"
    >
      <TreeItem
        v-for="(child, index) in children"
        :key="getItemKey(child, index)"
        :item="child"
        :level="level + 1"
        :selected-items="selectedItems"
        :expanded-items="expandedItems"
        :item-key="itemKey"
        :item-label="itemLabel"
        :item-children="itemChildren"
        :item-icon="itemIcon"
        :item-icon-class="itemIconClass"
        :variant="variant"
        :expand-on-click="expandOnClick"
        @click="bubbleEvent('click', $event)"
        @toggle-expand="bubbleEvent('toggle-expand', $event)"
        @select="bubbleEvent('select', $event)"
        @toggle="bubbleEvent('toggle', $event)"
        @create-dataset="bubbleEvent('create-dataset', $event)"
      >
        <template
          #default="{ item: slotItem, level: slotLevel, selected, expanded, toggle, select }"
        >
          <slot
            :item="slotItem"
            :level="slotLevel"
            :selected="selected"
            :expanded="expanded"
            :toggle="toggle"
            :select="select"
          />
        </template>
      </TreeItem>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import Button from '~/components/base/Button.vue';

const props = defineProps({
  item: {
    type: Object,
    required: true,
  },
  level: {
    type: Number,
    default: 0,
  },
  selectedItems: {
    type: Array,
    default: () => [],
  },
  expandedItems: {
    type: Array,
    default: () => [],
  },
  itemKey: {
    type: [String, Function],
    default: 'id',
  },
  itemLabel: {
    type: [String, Function],
    default: 'label',
  },
  itemChildren: {
    type: [String, Function],
    default: 'children',
  },
  itemIcon: {
    type: [String, Function],
    default: null,
  },
  itemIconClass: {
    type: [String, Function],
    default: null,
  },
  variant: {
    type: String,
    default: 'default',
  },
  expandOnClick: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(['click', 'toggle-expand', 'select', 'toggle', 'create-dataset']);

// 项目标签
const label = computed(() => {
  if (typeof props.itemLabel === 'function') {
    return props.itemLabel(props.item);
  }
  if (typeof props.item === 'object' && props.item[props.itemLabel]) {
    return props.item[props.itemLabel];
  }
  return String(props.item);
});

// 子项
const children = computed(() => {
  if (typeof props.itemChildren === 'function') {
    return props.itemChildren(props.item);
  }
  if (typeof props.item === 'object' && props.item[props.itemChildren]) {
    return props.item[props.itemChildren];
  }
  return [];
});

// 是否有子项
const hasChildren = computed(() => {
  return children.value && children.value.length > 0;
});

// 子项数量
const childrenCount = computed(() => {
  return children.value ? children.value.length : 0;
});

// 图标名称
const iconName = computed(() => {
  if (typeof props.itemIcon === 'function') {
    return props.itemIcon(props.item);
  }
  if (typeof props.itemIcon === 'string') {
    return props.itemIcon;
  }
  // 默认图标
  if (hasChildren.value) {
    return 'heroicons:folder';
  }
  return 'heroicons:document';
});

// 图标类
const iconClass = computed(() => {
  if (typeof props.itemIconClass === 'function') {
    return props.itemIconClass(props.item);
  }
  if (typeof props.itemIconClass === 'string') {
    return props.itemIconClass;
  }
  // 默认图标类
  if (hasChildren.value) {
    return 'text-warning';
  }
  return 'text-text-muted';
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

// 检查项目是否被选中
const isSelected = computed(() => {
  const key = getItemKey(props.item);
  return props.selectedItems.some(selected => getItemKey(selected) === key);
});

// 检查项目是否展开
const isExpanded = computed(() => {
  const key = getItemKey(props.item);
  return props.expandedItems.some(expanded => getItemKey(expanded) === key);
});

// 项目包装器样式类
const itemWrapperClasses = computed(() => {
  const baseClasses = [];

  if (props.variant === 'compact') {
    baseClasses.push('p-0.5');
  } else {
    baseClasses.push('p-0');
  }

  return baseClasses;
});

// 项目样式类
const itemClasses = computed(() => {
  const classes = [
    'flex cursor-pointer items-center gap-sm rounded-sm p-xs transition-all duration-fast',
    'text-text-primary',
    'tree-item',
  ];

  if (isSelected.value) {
    classes.push('selected');
  }

  return classes;
});

// 切换展开状态
const toggle = () => {
  emit('toggle-expand', props.item);
};

// 选择项目
const select = () => {
  emit('select', props.item);
};

// 处理点击事件
const handleClick = () => {
  // 先触发选择事件
  select();

  // 再触发点击事件
  emit('click', props.item);

  if (props.expandOnClick) {
    toggle();
  }
};

// 向上传播事件
const bubbleEvent = (event, data) => {
  emit(event, data);
};

// 格式化文件大小
const formatFileSize = bytes => {
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
};
</script>

<style scoped>
.tree-item {
  @apply border border-transparent;
}

.tree-item:hover {
  @apply border-border bg-background-secondary;
}

.tree-item.selected {
  @apply border-border-active bg-background-panel font-medium;
}
</style>
