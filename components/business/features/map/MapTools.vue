<template>
  <!-- 缩放按钮组 -->
  <div class="flex flex-col gap-px">
    <Button
      square
      title="放大"
      icon="heroicons:magnifying-glass-plus"
      @click="zoomIn"
    />
    <Button
      square
      title="缩小"
      icon="heroicons:magnifying-glass-minus"
      @click="zoomOut"
    />
  </div>
</template>

<script setup>
import Button from '~/components/base/Button.vue';

// 动作按钮事件
const zoomIn = () => {
  emit('zoom-in');
};

const zoomOut = () => {
  emit('zoom-out');
};

// 定义事件
const emit = defineEmits(['zoom-in', 'zoom-out']);
</script>
