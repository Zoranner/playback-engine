// @ts-check
import withNuxt from './.nuxt/eslint.config.mjs';
import eslintConfigPrettier from 'eslint-config-prettier';
import eslintPluginPrettier from 'eslint-plugin-prettier';

export default withNuxt({
  plugins: {
    prettier: eslintPluginPrettier,
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

    // Vue 3 特定规则
    'vue/multi-word-component-names': 'off',
    'vue/no-v-html': 'warn',
    'vue/require-default-prop': 'warn',
    'vue/require-prop-types': 'warn',
    'vue/prefer-import-from-vue': 'error',
    'vue/no-deprecated-scope-attribute': 'error',
    'vue/no-deprecated-slot-attribute': 'error',

    // TypeScript 规则
    '@typescript-eslint/no-unused-vars': [
      'warn',
      {
        'args': 'none',
        'ignoreRestSiblings': true,
        'varsIgnorePattern': '^_',
        'argsIgnorePattern': '^_'
      }
    ],
    '@typescript-eslint/no-explicit-any': 'warn',
    '@typescript-eslint/prefer-optional-chain': 'error',
    '@typescript-eslint/prefer-nullish-coalescing': 'error',
    '@typescript-eslint/no-non-null-assertion': 'warn',

    // Prettier 集成
    'prettier/prettier': 'error',

    // 禁用与 prettier 冲突的规则 - 这些将由 prettier 处理
    ...eslintConfigPrettier.rules,
  },

  // 忽略特定文件的规则
  ignores: [
    'dist/**',
    'node_modules/**',
    '.nuxt/**',
    '.output/**',
    'src-tauri/target/**',
  ],
});
