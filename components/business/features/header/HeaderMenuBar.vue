<template>
  <div
    class="flex flex-row-reverse items-center gap-0.5 rounded-sm border border-border bg-background-secondary p-thin"
  >
    <!-- 工程菜单 -->
    <DropdownMenu
      v-model="projectMenuOpen"
      label="工程"
      @open="() => openMenu('project')"
      @close="closeAllMenus"
    >
      <template #content="{ close }">
        <MenuItem
          icon="heroicons:folder-open"
          label="打开工程"
          shortcut="Ctrl+O"
          @click="openProject(close)"
        />
        <MenuItem
          icon="heroicons:x-circle"
          label="关闭工程"
          shortcut="Ctrl+W"
          :disabled="!isProjectLoaded"
          @click="closeProject(close)"
        />
        <MenuDivider />
        <MenuItem
          icon="heroicons:arrow-down-tray"
          label="导入数据"
          @click="importData(close)"
        />
        <MenuItem
          icon="heroicons:arrow-up-tray"
          label="导出数据"
          @click="exportData(close)"
        />
        <MenuDivider />
        <MenuItem
          icon="heroicons:power"
          label="退出"
          shortcut="Alt+F4"
          variant="danger"
          @click="exit(close)"
        />
      </template>
    </DropdownMenu>

    <!-- 视图菜单 -->
    <DropdownMenu
      v-model="viewMenuOpen"
      label="视图"
      @open="() => openMenu('view')"
      @close="closeAllMenus"
    >
      <template #content="{ close }">
        <MenuItem
          icon="heroicons:arrow-path"
          label="重置视图"
          shortcut="F5"
          @click="resetView(close)"
        />
        <MenuItem
          icon="heroicons:arrows-pointing-out"
          label="全屏显示"
          shortcut="F11"
          @click="fullscreen(close)"
        />
        <MenuDivider />
        <MenuItem
          icon="heroicons:squares-2x2"
          label="显示网格"
          @click="toggleGrid(close)"
        />
      </template>
    </DropdownMenu>

    <!-- 工具菜单 -->
    <DropdownMenu
      v-model="toolsMenuOpen"
      label="工具"
      @open="() => openMenu('tools')"
      @close="closeAllMenus"
    >
      <template #content="{ close }">
        <MenuItem
          icon="heroicons:cog-6-tooth"
          label="系统设置"
          @click="openSettings(close)"
        />
        <MenuDivider />
        <MenuItem
          icon="heroicons:information-circle"
          label="关于"
          @click="openAbout(close)"
        />
      </template>
    </DropdownMenu>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useProject } from '~/composables/useProject';
import DropdownMenu from '~/components/menu/DropdownMenu.vue';
import MenuItem from '~/components/menu/MenuItem.vue';
import MenuDivider from '~/components/menu/MenuDivider.vue';
import { invoke } from '@tauri-apps/api/core';

// 菜单状态
const projectMenuOpen = ref(false);
const viewMenuOpen = ref(false);
const toolsMenuOpen = ref(false);

// 使用工程管理
const { isProjectLoaded, openProject: openProjectAction, closeProject: closeProjectAction } = useProject();

// 打开指定菜单（关闭其他菜单）
const openMenu = (menuName: 'project' | 'view' | 'tools') => {
  // 先关闭所有菜单
  projectMenuOpen.value = false;
  viewMenuOpen.value = false;
  toolsMenuOpen.value = false;

  // 然后打开指定菜单
  switch (menuName) {
    case 'project':
      projectMenuOpen.value = true;
      break;
    case 'view':
      viewMenuOpen.value = true;
      break;
    case 'tools':
      toolsMenuOpen.value = true;
      break;
  }
};

// 关闭所有菜单
const closeAllMenus = () => {
  projectMenuOpen.value = false;
  viewMenuOpen.value = false;
  toolsMenuOpen.value = false;
};

// 菜单操作函数
const openProject = async (close: () => void) => {
  try {
    console.log('正在选择工程目录...');
    // 使用后端命令选择目录
    const selected = await invoke('select_project_directory');

    if (selected && typeof selected === 'string') {
      console.log('选择的目录:', selected);
      await openProjectAction(selected);
    } else {
      console.log('用户取消了选择');
    }
  } catch (err) {
    console.error('打开工程失败:', err);
  } finally {
    close();
  }
};

const closeProject = async (close: () => void) => {
  try {
    await closeProjectAction();
  } catch (err) {
    console.error('关闭工程失败:', err);
  } finally {
    close();
  }
};

const importData = (close: () => void) => {
  // TODO: 实现导入数据功能
  close();
};

const exportData = (close: () => void) => {
  // TODO: 实现导出数据功能
  close();
};

const exit = (close: () => void) => {
  // TODO: 实现退出应用功能
  close();
};

const resetView = (close: () => void) => {
  // TODO: 实现重置视图功能
  close();
};

const fullscreen = (close: () => void) => {
  // TODO: 实现全屏显示功能
  close();
};

const toggleGrid = (close: () => void) => {
  // TODO: 实现切换网格显示功能
  close();
};

const openSettings = (close: () => void) => {
  // TODO: 实现打开设置功能
  close();
};

const openAbout = (close: () => void) => {
  // TODO: 实现关于对话框功能
  close();
};
</script>
