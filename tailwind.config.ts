import type { Config } from 'tailwindcss';

export default <Partial<Config>>{
  content: [
    './components/**/*.{vue,js,ts}',
    './layouts/**/*.vue',
    './pages/**/*.vue',
    './composables/**/*.{js,ts}',
    './plugins/**/*.{js,ts}',
    './app.vue',
  ],
  theme: {
    extend: {
      colors: {
        // 深色主题色系
        primary: {
          DEFAULT: '#0ea5e9',
          50: '#f0f9ff',
          100: '#e0f2fe',
          200: '#bae6fd',
          300: '#7dd3fc',
          400: '#38bdf8',
          500: '#0ea5e9',
          600: '#0284c7',
          700: '#0369a1',
          800: '#075985',
          900: '#0c4a6e',
        },
        // 背景色系
        background: {
          primary: '#0f1419',
          secondary: '#1a1f2e',
          tertiary: '#252b3a',
          panel: '#1e2532',
          header: '#161b26',
          card: '#1e2532',
          overlay: 'rgba(15, 20, 25, 0.8)',
        },
        // 边框色系
        border: {
          DEFAULT: '#334155',
          light: '#475569',
          active: '#0ea5e9',
          divider: '#2d3748',
        },
        // 文字色系
        text: {
          primary: '#f1f5f9',
          secondary: '#cbd5e1',
          muted: '#94a3b8',
          accent: '#38bdf8',
        },
        // 状态色系
        success: {
          DEFAULT: '#10b981',
          light: '#34d399',
          dark: '#059669',
        },
        warning: {
          DEFAULT: '#f59e0b',
          light: '#fbbf24',
          dark: '#d97706',
        },
        danger: {
          DEFAULT: '#ef4444',
          light: '#f87171',
          dark: '#dc2626',
        },
        info: {
          DEFAULT: '#3b82f6',
        },
      },
      // 字体系统
      fontFamily: {
        main: [
          'Microsoft YaHei',
          'PingFang SC',
          '-apple-system',
          'BlinkMacSystemFont',
          'Segoe UI',
          'Roboto',
          'sans-serif',
        ],
        mono: ['Consolas', 'Monaco', 'Courier New', 'monospace'],
      },
      // 间距系统
      spacing: {
        xs: '4px',
        sm: '8px',
        md: '12px',
        lg: '16px',
        xl: '20px',
        '2xl': '24px',
        xxl: '32px',
        thin: '2px',
        divider: '1px',
      },
      // 阴影系统
      boxShadow: {
        xs: '0 1px 2px rgba(0, 0, 0, 0.2)',
        sm: '0 1px 2px rgba(0, 0, 0, 0.4)',
        md: '0 4px 6px rgba(0, 0, 0, 0.3)',
        lg: '0 10px 15px rgba(0, 0, 0, 0.4)',
        xl: '0 20px 25px rgba(0, 0, 0, 0.5)',
        inner: 'inset 0 1px 3px rgba(0, 0, 0, 0.3)',
        glow: '0 0 8px rgba(56, 189, 248, 0.3)',
        'glow-strong': '0 0 12px rgba(56, 189, 248, 0.4)',
        'glow-subtle': '0 0 4px rgba(56, 189, 248, 0.2)',
        overlay: '0 8px 32px rgba(0, 0, 0, 0.6)',
      },
      // 过渡动画
      transitionDuration: {
        fast: '0.15s',
        normal: '0.25s',
        slow: '0.35s',
      },
      // 尺寸系统
      height: {
        header: '56px',
        status: '28px',
        '5.5': '22px', // Toggle 轨道高度
        divider: '1px',
      },
      width: {
        panel: '280px',
        '4.5': '18px', // Toggle 滑块宽度
      },
      // 圆角系统
      borderRadius: {
        xs: '2px',
        sm: '3px',
        md: '6px',
        lg: '8px',
      },
      // Z-index 系统
      zIndex: {
        header: '40',
        dropdown: '50',
        modal: '100',
      },
      // 渐变系统
      backgroundImage: {
        'gradient-default': 'linear-gradient(145deg, #252b3a, #1a1f2e)',
        'gradient-primary': 'linear-gradient(145deg, #0ea5e9, #0284c7)',
        'gradient-success': 'linear-gradient(145deg, #10b981, #059669)',
        'gradient-warning': 'linear-gradient(145deg, #f59e0b, #d97706)',
        'gradient-danger': 'linear-gradient(145deg, #ef4444, #dc2626)',
        'gradient-header': 'linear-gradient(135deg, #161b26, #1a1f2e)',
        'gradient-divider': 'linear-gradient(90deg, #334155, #475569)',
      },
      // 字体大小
      fontSize: {
        body: ['14px', '1.5'],
        title: ['16px', '1.4'],
        subtitle: ['13px', '1.4'],
        caption: ['12px', '1.3'],
      },
    },
  },
  plugins: [],
};
