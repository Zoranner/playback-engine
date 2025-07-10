import { ref } from 'vue';
import type { Target } from '~/types';

export const useTargets = () => {
  const targets = ref<Target[]>([
    {
      id: 'CKB_1435.763',
      label: '1749',
      x: 320,
      y: 285,
      color: '#00d9ff',
      strokeColor: '#0ea5e9',
      distance: '12.5',
      bearing: '045',
      status: 'friendly',
    },
    {
      id: 'CKB_1435.764',
      label: '743',
      x: 580,
      y: 310,
      color: '#4ade80',
      strokeColor: '#22c55e',
      distance: '8.2',
      bearing: '120',
      status: 'neutral',
    },
    {
      id: 'CKB_1435.765',
      label: '2291',
      x: 450,
      y: 200,
      color: '#f59e0b',
      strokeColor: '#d97706',
      distance: '15.7',
      bearing: '280',
      status: 'unknown',
    },
  ]);

  const selectTarget = (_target: Target) => {
    // TODO: 实现目标选择逻辑
  };

  return {
    targets,
    selectTarget,
  };
};
