import { ref, computed, readonly } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ProjectInfo, AppInfo } from '~/types/project';

export const useProject = () => {
  // 状态
  const currentProject = ref<ProjectInfo | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  // 计算属性
  const isProjectLoaded = computed(() => currentProject.value !== null);
  const projectName = computed(() => currentProject.value?.name || '');
  const projectPath = computed(() => currentProject.value?.path || '');
  const fileCount = computed(() => currentProject.value?.fileCount || 0);
  const totalDurationSeconds = computed(() => {
    if (!currentProject.value) return 0;
    return currentProject.value.totalDuration / 1_000_000_000;
  });

  // 格式化显示的总时长
  const formattedDuration = computed(() => {
    const seconds = totalDurationSeconds.value;
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
    }
    return `${minutes}:${secs.toString().padStart(2, '0')}`;
  });

  // 打开工程
  const openProject = async (path: string): Promise<ProjectInfo | null> => {
    isLoading.value = true;
    error.value = null;

    try {
      console.log('正在打开工程:', path);
      const projectInfo = await invoke<ProjectInfo>('open_project', { path });

      currentProject.value = projectInfo;
      console.log('工程打开成功:', projectInfo);

      return projectInfo;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      error.value = errorMessage;
      console.error('打开工程失败:', errorMessage);
      return null;
    } finally {
      isLoading.value = false;
    }
  };

  // 关闭工程
  const closeProject = async (): Promise<boolean> => {
    if (!isProjectLoaded.value) return true;

    isLoading.value = true;
    error.value = null;

    try {
      await invoke('close_project');
      currentProject.value = null;
      console.log('工程已关闭');
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      error.value = errorMessage;
      console.error('关闭工程失败:', errorMessage);
      return false;
    } finally {
      isLoading.value = false;
    }
  };

  // 获取工程元数据
  const getProjectMetadata = async (): Promise<ProjectInfo | null> => {
    try {
      const projectInfo = await invoke<ProjectInfo | null>('get_project_metadata');
      if (projectInfo) {
        currentProject.value = projectInfo;
      }
      return projectInfo;
    } catch (err) {
      console.error('获取工程元数据失败:', err);
      return null;
    }
  };

  // 验证工程目录
  const validateProjectDirectory = async (path: string): Promise<boolean> => {
    try {
      const isValid = await invoke<boolean>('validate_project_directory', { path });
      return isValid;
    } catch (err) {
      console.error('验证工程目录失败:', err);
      return false;
    }
  };

  // 测试后端连接
  const testConnection = async (): Promise<boolean> => {
    try {
      const response = await invoke<string>('test_connection');
      console.log('后端连接测试:', response);
      return true;
    } catch (err) {
      console.error('后端连接测试失败:', err);
      return false;
    }
  };

  // 获取应用信息
  const getAppInfo = async (): Promise<AppInfo | null> => {
    try {
      const appInfo = await invoke<AppInfo>('get_app_info');
      console.log('应用信息:', appInfo);
      return appInfo;
    } catch (err) {
      console.error('获取应用信息失败:', err);
      return null;
    }
  };

  // 清除错误
  const clearError = () => {
    error.value = null;
  };

  // 重置状态
  const resetState = () => {
    currentProject.value = null;
    isLoading.value = false;
    error.value = null;
  };

  return {
    // 状态
    currentProject: readonly(currentProject),
    isLoading: readonly(isLoading),
    error: readonly(error),

    // 计算属性
    isProjectLoaded,
    projectName,
    projectPath,
    fileCount,
    totalDurationSeconds,
    formattedDuration,

    // 方法
    openProject,
    closeProject,
    getProjectMetadata,
    validateProjectDirectory,
    testConnection,
    getAppInfo,
    clearError,
    resetState,
  };
};
