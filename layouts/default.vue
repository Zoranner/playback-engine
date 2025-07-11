<template>
  <div
    class="relative flex h-screen w-screen flex-col overflow-hidden bg-background-primary font-main text-body text-text-primary"
  >
    <!-- 上层：顶部标题栏 -->
    <TopHeader class="flex-none" />

    <!-- 中层：主要内容区域 -->
    <div class="flex h-full flex-1 flex-col overflow-hidden">
      <slot />
    </div>

    <!-- 下层：底部状态栏 -->
    <StatusBar class="flex-none" />
  </div>
</template>

<script setup>
import TopHeader from '~/components/business/layout/TopHeader.vue';
import StatusBar from '~/components/business/layout/StatusBar.vue';
</script>
