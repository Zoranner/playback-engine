<template>
  <header
    class="relative z-header flex h-header items-center justify-between border-b border-border bg-gradient-header px-xl shadow-md"
  >
    <div class="flex flex-1 items-center gap-2xl">
      <HeaderLogo />
    </div>
    <div class="flex flex-1 justify-center">
      <PlaybackControls />
    </div>
    <div class="flex flex-1 items-center justify-end gap-lg">
      <HeaderMenuBar />
    </div>
  </header>
</template>

<script setup>
import HeaderLogo from './HeaderLogo.vue';
import PlaybackControls from './PlaybackControls.vue';
import HeaderMenuBar from './HeaderMenuBar.vue';
</script>
