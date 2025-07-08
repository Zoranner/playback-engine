<template>
  <header class="top-header">
    <div class="header-left">
      <HeaderLogo />
    </div>
    <div class="header-center">
      <HeaderPlaybackControls />
    </div>
    <div class="header-right">
      <HeaderMenuBar />
    </div>
  </header>
</template>

<script setup>
import HeaderLogo from './HeaderLogo.vue';
import HeaderPlaybackControls from './HeaderPlaybackControls.vue';
import HeaderMenuBar from './HeaderMenuBar.vue';
</script>

<style scoped>
.top-header {
  height: var(--header-height);
  background: linear-gradient(145deg, var(--header-bg), var(--secondary-bg));
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--spacing-xl);
  box-shadow: var(--shadow-md);
  position: relative;
  z-index: 100;
}

.top-header::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-color-light), transparent);
}

.header-left {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--spacing-2xl);
}

.header-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.header-right {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--spacing-lg);
}
</style> 