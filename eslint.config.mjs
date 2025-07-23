// @ts-check
import withNuxt from './.nuxt/eslint.config.mjs';
import eslintConfigPrettier from 'eslint-config-prettier';
import eslintPluginPrettier from 'eslint-plugin-prettier';

export default withNuxt(
  {
    plugins: {
      prettier: eslintPluginPrettier,
    },
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      // 基础代码质量规则
      'no-console': 'off',
      'no-debugger': 'error',
      'no-unused-vars': 'off',
      'prefer-const': 'error',
      'no-var': 'error',
      'object-shorthand': 'error',
      'prefer-template': 'error',
      'prefer-arrow-callback': 'error',
      'no-duplicate-imports': 'error',

      // Prettier 集成
      'prettier/prettier': 'error',

      // 禁用与 prettier 冲突的规则 - 这些将由 prettier 处理
      ...eslintConfigPrettier.rules,
    },
  },
  // Vue 3 特定规则
  {
    files: ['**/*.vue'],
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/no-v-html': 'warn',
      'vue/require-default-prop': 'warn',
      'vue/require-prop-types': 'warn',
      'vue/prefer-import-from-vue': 'error',
      'vue/no-deprecated-scope-attribute': 'error',
      'vue/no-deprecated-slot-attribute': 'error',
    },
  },
  // TypeScript 规则
  {
    files: ['**/*.ts', '**/*.tsx', '**/*.vue'],
    rules: {
      '@typescript-eslint/no-unused-vars': [
        'warn',
        {
          args: 'none',
          ignoreRestSiblings: true,
          varsIgnorePattern: '^_',
          argsIgnorePattern: '^_',
        },
      ],
      '@typescript-eslint/no-explicit-any': 'warn',
      '@typescript-eslint/prefer-optional-chain': 'warn',
      '@typescript-eslint/prefer-nullish-coalescing': 'warn',
      '@typescript-eslint/no-non-null-assertion': 'warn',
    },
  },
  // 全局忽略配置
  {
    ignores: [
      // IDE 和编辑器目录
      '.claude/**',
      '.cursor/**',
      '.vscode/**',

      // Nuxt 生成文件
      '.nuxt/**',
      '.output/**',

      // 依赖目录
      'node_modules/**',

      // Tauri 相关
      'src-tauri/**',

      // 构建和生成文件
      '**/*.min.js',
      '**/*-codegen-*',
      '**/dist/**',
      '**/coverage/**',

      // 配置文件（如果需要的话）
      // '*.config.js',
      // '*.config.mjs',
      // '*.config.ts',
    ],
  }
);
