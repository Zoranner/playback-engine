<template>
  <div
    class="flex flex-row-reverse items-center gap-0.5 rounded-sm border border-border bg-background-secondary p-thin"
  >
    <div
      class="relative flex cursor-pointer select-none items-center gap-xs whitespace-nowrap rounded-xs bg-transparent px-sm py-xs transition-all duration-fast hover:bg-background-tertiary hover:shadow-glow-subtle"
      :class="{ 'bg-background-tertiary shadow-glow-subtle': activeMenu === 'project' }"
      @click="toggleMenu('project')"
    >
      <span class="font-medium text-text-primary">工程</span>
      <Icon
        name="heroicons:chevron-down"
        class="text-text-secondary transition-transform duration-fast"
        :class="{ 'rotate-180': activeMenu === 'project' }"
      />
      <div
        v-if="activeMenu === 'project'"
        class="absolute right-0 top-[calc(100%+4px)] z-dropdown min-w-[180px] overflow-hidden rounded-md border border-border bg-background-secondary shadow-md backdrop-blur-md"
      >
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="openProject"
        >
          <Icon name="heroicons:folder-open" class="shrink-0" />
          <span class="flex-1">打开工程</span>
          <span class="ml-auto text-text-secondary opacity-60">Ctrl+O</span>
        </div>
        <div
          class="pointer-events-none relative flex cursor-not-allowed cursor-pointer items-center gap-sm px-md py-sm opacity-50 transition-colors duration-fast hover:bg-background-tertiary"
          :class="{ 'pointer-events-none cursor-not-allowed opacity-50': !isProjectLoaded }"
          @click.stop="closeProject"
        >
          <Icon name="heroicons:x-circle" class="shrink-0" />
          <span class="flex-1">关闭工程</span>
          <span class="ml-auto text-text-secondary opacity-60">Ctrl+W</span>
        </div>
        <div class="mx-0 my-xs h-divider bg-border" />
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="importData"
        >
          <Icon name="heroicons:arrow-down-tray" class="shrink-0" />
          <span class="flex-1">导入数据</span>
        </div>
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="exportData"
        >
          <Icon name="heroicons:arrow-up-tray" class="shrink-0" />
          <span class="flex-1">导出数据</span>
        </div>
        <div class="mx-0 my-xs h-divider bg-border" />
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm text-danger transition-colors duration-fast hover:bg-danger hover:text-background-primary"
          @click.stop="exit"
        >
          <Icon name="heroicons:power" class="shrink-0" />
          <span class="flex-1">退出</span>
          <span class="ml-auto text-text-secondary opacity-60">Alt+F4</span>
        </div>
      </div>
    </div>

    <div
      class="relative flex cursor-pointer select-none items-center gap-xs whitespace-nowrap rounded-xs bg-transparent px-sm py-xs transition-all duration-fast hover:bg-background-tertiary hover:shadow-glow-subtle"
      :class="{ 'bg-background-tertiary shadow-glow-subtle': activeMenu === 'view' }"
      @click="toggleMenu('view')"
    >
      <span class="font-medium text-text-primary">视图</span>
      <Icon
        name="heroicons:chevron-down"
        class="text-text-secondary transition-transform duration-fast"
        :class="{ 'rotate-180': activeMenu === 'view' }"
      />
      <div
        v-if="activeMenu === 'view'"
        class="absolute right-0 top-[calc(100%+4px)] z-dropdown min-w-[180px] overflow-hidden rounded-md border border-border bg-background-secondary shadow-md backdrop-blur-md"
      >
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="resetView"
        >
          <Icon name="heroicons:arrow-path" class="shrink-0" />
          <span class="flex-1">重置视图</span>
          <span class="ml-auto text-text-secondary opacity-60">F5</span>
        </div>
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="fullscreen"
        >
          <Icon name="heroicons:arrows-pointing-out" class="shrink-0" />
          <span class="flex-1">全屏显示</span>
          <span class="ml-auto text-text-secondary opacity-60">F11</span>
        </div>
        <div class="mx-0 my-xs h-divider bg-border" />
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="toggleGrid"
        >
          <Icon name="heroicons:squares-2x2" class="shrink-0" />
          <span class="flex-1">显示网格</span>
        </div>
      </div>
    </div>

    <div
      class="relative flex cursor-pointer select-none items-center gap-xs whitespace-nowrap rounded-xs bg-transparent px-sm py-xs transition-all duration-fast hover:bg-background-tertiary hover:shadow-glow-subtle"
      :class="{ 'bg-background-tertiary shadow-glow-subtle': activeMenu === 'tools' }"
      @click="toggleMenu('tools')"
    >
      <span class="font-medium text-text-primary">工具</span>
      <Icon
        name="heroicons:chevron-down"
        class="text-text-secondary transition-transform duration-fast"
        :class="{ 'rotate-180': activeMenu === 'tools' }"
      />
      <div
        v-if="activeMenu === 'tools'"
        class="absolute right-0 top-[calc(100%+4px)] z-dropdown min-w-[180px] overflow-hidden rounded-md border border-border bg-background-secondary shadow-md backdrop-blur-md"
      >
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="openSettings"
        >
          <Icon name="heroicons:cog-6-tooth" class="shrink-0" />
          <span class="flex-1">系统设置</span>
        </div>
        <div class="mx-0 my-xs h-divider bg-border" />
        <div
          class="relative flex cursor-pointer items-center gap-sm px-md py-sm transition-colors duration-fast hover:bg-background-tertiary"
          @click.stop="openAbout"
        >
          <Icon name="heroicons:information-circle" class="shrink-0" />
          <span class="flex-1">关于</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';

const activeMenu = ref<string | null>(null);

// 使用composables
const {
  isProjectLoaded,
  openProject: openProjectAction,
  closeProject: closeProjectAction,
} = useProject();

const toggleMenu = (menu: string) => {
  activeMenu.value = activeMenu.value === menu ? null : menu;
};

// 菜单操作函数
const openProject = async () => {
  try {
    // 使用Tauri的dialog插件打开文件夹选择对话框
    const selected = await open({
      directory: true,
      title: '选择回放数据工程目录',
    });

    if (selected && typeof selected === 'string') {
      await openProjectAction(selected);
    }
  } catch (err) {
    console.error('打开工程失败:', err);
  } finally {
    activeMenu.value = null;
  }
};

const closeProject = async () => {
  try {
    await closeProjectAction();
  } catch (err) {
    console.error('关闭工程失败:', err);
  } finally {
    activeMenu.value = null;
  }
};

const importData = () => {
  // TODO: 实现导入数据功能
  activeMenu.value = null;
};

const exportData = () => {
  // TODO: 实现导出数据功能
  activeMenu.value = null;
};

const exit = () => {
  // TODO: 实现退出应用功能
  activeMenu.value = null;
};

const resetView = () => {
  // TODO: 实现重置视图功能
  activeMenu.value = null;
};

const fullscreen = () => {
  // TODO: 实现全屏显示功能
  activeMenu.value = null;
};

const toggleGrid = () => {
  // TODO: 实现切换网格显示功能
  activeMenu.value = null;
};

const openSettings = () => {
  // TODO: 实现打开设置功能
  activeMenu.value = null;
};

const openAbout = () => {
  // TODO: 实现关于对话框功能
  activeMenu.value = null;
};
</script>
