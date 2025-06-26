<template>
  <div class="menu-bar">
    <div class="menu-item" :class="{ active: activeMenu === 'project' }" @click="toggleMenu('project')">
      <span class="menu-title">工程</span>
      <Icon name="heroicons:chevron-down" class="menu-arrow" />
      <div v-if="activeMenu === 'project'" class="dropdown-menu">
        <div class="menu-option" @click.stop="openProject">
          <Icon name="heroicons:folder-open" />
          <span>打开工程</span>
          <span class="menu-shortcut">Ctrl+O</span>
        </div>
        <div class="menu-option" @click.stop="closeProject">
          <Icon name="heroicons:x-circle" />
          <span>关闭工程</span>
          <span class="menu-shortcut">Ctrl+W</span>
        </div>
        <div class="menu-divider"/>
        <div class="menu-option" @click.stop="importData">
          <Icon name="heroicons:arrow-down-tray" />
          <span>导入数据</span>
        </div>
        <div class="menu-option" @click.stop="exportData">
          <Icon name="heroicons:arrow-up-tray" />
          <span>导出数据</span>
        </div>
        <div class="menu-divider"/>
        <div class="menu-option danger" @click.stop="exit">
          <Icon name="heroicons:power" />
          <span>退出</span>
          <span class="menu-shortcut">Alt+F4</span>
        </div>
      </div>
    </div>

    <div class="menu-item" :class="{ active: activeMenu === 'view' }" @click="toggleMenu('view')">
      <span class="menu-title">视图</span>
      <Icon name="heroicons:chevron-down" class="menu-arrow" />
      <div v-if="activeMenu === 'view'" class="dropdown-menu">
        <div class="menu-option" @click.stop="resetView">
          <Icon name="heroicons:arrow-path" />
          <span>重置视图</span>
          <span class="menu-shortcut">F5</span>
        </div>
        <div class="menu-option" @click.stop="fullscreen">
          <Icon name="heroicons:arrows-pointing-out" />
          <span>全屏显示</span>
          <span class="menu-shortcut">F11</span>
        </div>
        <div class="menu-divider"/>
        <div class="menu-option" @click.stop="toggleGrid">
          <Icon name="heroicons:squares-2x2" />
          <span>显示网格</span>
        </div>
      </div>
    </div>

    <div class="menu-item" :class="{ active: activeMenu === 'tools' }" @click="toggleMenu('tools')">
      <span class="menu-title">工具</span>
      <Icon name="heroicons:chevron-down" class="menu-arrow" />
      <div v-if="activeMenu === 'tools'" class="dropdown-menu">
        <div class="menu-option" @click.stop="openSettings">
          <Icon name="heroicons:cog-6-tooth" />
          <span>系统设置</span>
        </div>
        <div class="menu-divider"/>
        <div class="menu-option" @click.stop="openAbout">
          <Icon name="heroicons:information-circle" />
          <span>关于</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const activeMenu = ref(null)

const toggleMenu = (menu) => {
  activeMenu.value = activeMenu.value === menu ? null : menu
}

// 菜单操作函数
const openProject = () => {
  console.log('打开工程')
  activeMenu.value = null
}

const closeProject = () => {
  console.log('关闭工程')
  activeMenu.value = null
}

const importData = () => {
  console.log('导入数据')
  activeMenu.value = null
}

const exportData = () => {
  console.log('导出数据')
  activeMenu.value = null
}

const exit = () => {
  console.log('退出应用')
  activeMenu.value = null
}

const resetView = () => {
  console.log('重置视图')
  activeMenu.value = null
}

const fullscreen = () => {
  console.log('全屏显示')
  activeMenu.value = null
}

const toggleGrid = () => {
  console.log('切换网格显示')
  activeMenu.value = null
}

const openSettings = () => {
  console.log('打开设置')
  activeMenu.value = null
}

const openAbout = () => {
  console.log('关于对话框')
  activeMenu.value = null
}
</script>

<style scoped>
.menu-bar {
  display: flex;
  align-items: center;
  gap: 2px;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-sm);
  padding: 2px;
  flex-direction: row-reverse;
}

.menu-item {
  position: relative;
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  background: transparent;
  border-radius: var(--radius-xs);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-size: 11px;
  user-select: none;
  white-space: nowrap;
}

.menu-item:hover,
.menu-item.active {
  background: var(--tertiary-bg);
  box-shadow: var(--glow-subtle);
}

.menu-title {
  font-weight: 500;
  color: var(--text-primary);
}

.menu-arrow {
  font-size: 8px;
  color: var(--text-secondary);
  transition: transform var(--transition-fast);
}

.menu-item.active .menu-arrow {
  transform: rotate(180deg);
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 4px);
  right: 0;
  background: var(--secondary-bg);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
  min-width: 180px;
  z-index: 1000;
  overflow: hidden;
  backdrop-filter: blur(8px);
}

.menu-option {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  font-size: 11px;
  cursor: pointer;
  transition: background-color var(--transition-fast);
  position: relative;
}

.menu-option:hover {
  background: var(--tertiary-bg);
}

.menu-option.danger {
  color: var(--danger-color);
}

.menu-option.danger:hover {
  background: var(--danger-color);
  color: var(--primary-bg);
}

.menu-shortcut {
  margin-left: auto;
  font-size: 9px;
  color: var(--text-secondary);
  opacity: 0.7;
}

.menu-divider {
  height: 1px;
  background: var(--border-color);
  margin: var(--spacing-xs) 0;
}

.menu-option svg {
  font-size: 12px;
  flex-shrink: 0;
}

.menu-option span:nth-child(2) {
  flex: 1;
}
</style> 