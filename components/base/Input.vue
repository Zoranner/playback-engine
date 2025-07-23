<template>
  <div class="mb-md">
    <label
      v-if="label"
      class="mb-xs block text-sm font-medium text-text-secondary"
    >
      {{ label }}
    </label>
    <input
      :value="modelValue"
      :type="type"
      :placeholder="placeholder"
      :disabled="disabled"
      class="w-full rounded-sm border border-border bg-background-secondary px-sm py-xs text-text-primary transition-colors duration-fast focus:border-border-active focus:outline-none focus:ring-1 focus:ring-primary"
      :class="{ 'cursor-not-allowed opacity-50': disabled }"
      @input="$emit('update:modelValue', $event.target.value)"
      @keyup.enter="$emit('enter')"
    />
    <div
      v-if="error"
      class="mt-xs text-sm text-danger"
    >
      {{ error }}
    </div>
    <div
      v-else-if="helpText"
      class="mt-xs text-sm text-text-muted"
    >
      {{ helpText }}
    </div>
  </div>
</template>

<script setup>
defineProps({
  modelValue: {
    type: String,
    default: '',
  },
  label: {
    type: String,
    default: '',
  },
  type: {
    type: String,
    default: 'text',
  },
  placeholder: {
    type: String,
    default: '',
  },
  error: {
    type: String,
    default: '',
  },
  helpText: {
    type: String,
    default: '',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
});

defineEmits(['update:modelValue', 'enter']);
</script>
