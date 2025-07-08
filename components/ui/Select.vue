<template>
  <div class="select-container" :class="containerClass">
    <label v-if="label" class="select-label">{{ label }}</label>
    <div class="select-wrapper" :class="{ 'select-wrapper--open': isOpen }">
      <div 
        class="select-trigger"
        :class="triggerClass"
        :tabindex="disabled ? -1 : 0"
        @click="toggleOpen"
        @keydown.enter.space.prevent="toggleOpen"
        @keydown.esc="closeSelect"
      >
        <span class="select-value">{{ displayValue }}</span>
        <Icon name="heroicons:chevron-down" size="12" class="select-arrow" />
      </div>
      
      <Transition name="select-dropdown">
        <div v-if="isOpen" ref="dropdownRef" class="select-dropdown">
          <div 
            v-for="option in options" 
            :key="option.value"
            class="select-option"
            :class="{ 
              'select-option--selected': option.value === modelValue,
              'select-option--disabled': option.disabled 
            }"
            @click="selectOption(option)"
          >
            <Icon v-if="option.icon" :name="option.icon" size="14" class="option-icon" />
            <span class="option-text">{{ option.label }}</span>
            <Icon v-if="option.value === modelValue" name="heroicons:check" size="12" class="option-check" />
          </div>
        </div>
      </Transition>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';


const props = defineProps({
  modelValue: {
    type: [String, Number, Boolean],
    required: true
  },
  options: {
    type: Array,
    required: true,
    validator: (options) => {
      return options.every(option => 
        typeof option === 'object' && 
        'value' in option && 
        'label' in option
      );
    }
  },
  label: {
    type: String,
    default: ''
  },
  placeholder: {
    type: String,
    default: '请选择'
  },
  disabled: {
    type: Boolean,
    default: false
  },
  size: {
    type: String,
    default: 'medium',
    validator: (value) => ['small', 'medium', 'large'].includes(value)
  },
  variant: {
    type: String,
    default: 'default',
    validator: (value) => ['default', 'ghost'].includes(value)
  }
});

const emit = defineEmits(['update:modelValue', 'change']);

const isOpen = ref(false);
const dropdownRef = ref(null);

const containerClass = computed(() => [
  `select-container--${props.size}`,
  `select-container--${props.variant}`,
  { 'select-container--disabled': props.disabled }
]);

const triggerClass = computed(() => [
  'select-trigger',
  `select-trigger--${props.size}`,
  `select-trigger--${props.variant}`,
  { 
    'select-trigger--disabled': props.disabled,
    'select-trigger--open': isOpen.value
  }
]);

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

const selectOption = (option) => {
  if (option.disabled) return;
  
  emit('update:modelValue', option.value);
  emit('change', option);
  closeSelect();
};

const handleClickOutside = (event) => {
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

watch(isOpen, (newValue) => {
  if (newValue) {
    document.addEventListener('click', handleClickOutside);
  }
});
</script>

<style scoped>
.select-container {
  position: relative;
  width: 100%;
}

.select-label {
  display: block;
  font-size: 11px;
  color: var(--text-secondary);
  margin-bottom: var(--spacing-xs);
  font-weight: 500;
}

.select-wrapper {
  position: relative;
}

.select-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  color: var(--text-primary);
  cursor: pointer;
  transition: all var(--transition-fast);
  user-select: none;
  position: relative;
}

.select-trigger:hover:not(.select-trigger--disabled) {
  border-color: var(--border-color-light);
  background: var(--tertiary-bg);
}

.select-trigger--open {
  border-color: var(--border-color-active);
  box-shadow: var(--glow-subtle);
}

.select-trigger--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 尺寸变体 */
.select-trigger--small {
  height: 24px;
  padding: 0 var(--spacing-sm);
  font-size: 11px;
}

.select-trigger--medium {
  height: 32px;
  padding: 0 var(--spacing-md);
  font-size: 13px;
}

.select-trigger--large {
  height: 40px;
  padding: 0 var(--spacing-lg);
  font-size: 14px;
}

/* 变体样式 */
.select-trigger--ghost {
  background: transparent;
  border-color: transparent;
}

.select-trigger--ghost:hover:not(.select-trigger--disabled) {
  background: var(--secondary-bg);
  border-color: var(--border-color);
}

.select-value {
  flex: 1;
  text-align: left;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.select-arrow {
  color: var(--text-secondary);
  transition: transform var(--transition-fast);
  flex-shrink: 0;
  margin-left: var(--spacing-xs);
}

.select-trigger--open .select-arrow {
  transform: rotate(180deg);
}

.select-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  left: 0;
  right: 0;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
  z-index: 1000;
  max-height: 200px;
  overflow-y: auto;
  backdrop-filter: blur(8px);
}

.select-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  cursor: pointer;
  transition: background-color var(--transition-fast);
  font-size: 13px;
}

.select-option:hover:not(.select-option--disabled) {
  background: var(--tertiary-bg);
}

.select-option--selected {
  background: var(--border-color);
  color: var(--text-accent);
}

.select-option--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.option-icon {
  color: var(--text-secondary);
  flex-shrink: 0;
}

.option-text {
  flex: 1;
}

.option-check {
  color: var(--text-accent);
  flex-shrink: 0;
}

/* 下拉动画 */
.select-dropdown-enter-active,
.select-dropdown-leave-active {
  transition: all var(--transition-fast);
  transform-origin: top;
}

.select-dropdown-enter-from {
  opacity: 0;
  transform: scaleY(0.8) translateY(-8px);
}

.select-dropdown-leave-to {
  opacity: 0;
  transform: scaleY(0.8) translateY(-8px);
}
</style> 