<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm"
      @click="$emit('update:visible', false)"
    >
      <div
        class="w-full max-w-md overflow-hidden rounded-lg border border-border bg-background-secondary shadow-xl"
        @click.stop
      >
        <div class="border-b border-border-divider bg-background-header px-md py-sm">
          <h3 class="m-0 font-semibold text-text-primary">
            {{ title }}
          </h3>
        </div>

        <div class="p-md">
          <slot />
        </div>

        <div
          class="flex items-center justify-end gap-sm border-t border-border-divider bg-background-header px-md py-sm"
        >
          <Button
            variant="ghost"
            @click="$emit('update:visible', false)"
          >
            {{ cancelText }}
          </Button>
          <Button
            variant="primary"
            @click="$emit('confirm')"
          >
            {{ confirmText }}
          </Button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup>
import Button from '~/components/base/Button.vue';

defineProps({
  visible: {
    type: Boolean,
    default: false,
  },
  title: {
    type: String,
    default: '',
  },
  confirmText: {
    type: String,
    default: '确认',
  },
  cancelText: {
    type: String,
    default: '取消',
  },
});

defineEmits(['update:visible', 'confirm']);
</script>
