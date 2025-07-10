<template>
  <div
    class="relative flex h-screen w-screen flex-col overflow-hidden bg-background-primary font-main text-body text-text-primary"
  >
    <!-- 主要内容 -->
    <div class="flex h-full w-full flex-col overflow-hidden">
      <div class="flex flex-1 flex-col overflow-hidden">
        <slot />
      </div>
      <StatusBar />
    </div>
  </div>
</template>

<script setup>
import StatusBar from '~/components/business/StatusBar.vue';
</script>
