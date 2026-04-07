import pluginJs from '@eslint/js';
import tseslint from 'typescript-eslint';

export default [
    pluginJs.configs.recommended,
    ...tseslint.configs.recommended,
    {
        languageOptions: {
            ecmaVersion: 'latest',
            sourceType: 'module',
        },
        rules: {
            'no-irregular-whitespace': ['error', { skipComments: true }],
            '@typescript-eslint/no-explicit-any': 'off',
            'no-useless-escape': 'off',
            'no-irregular-whitespace': 'off',
            '@typescript-eslint/no-unused-expressions': [
                'error',
                {
                    allowShortCircuit: true,
                    allowTernary: true,
                },
            ],
            quotes: ['error', 'single'],
            semi: ['error', 'always'],
            indent: ['error', 4],
        },
    },
    {
        files: ['**/*.{js,mjs,cjs,ts}'],
    },
    {
        files: ['**/*.js'],
        languageOptions: {
            sourceType: 'commonjs',
        },
    },
    {
        ignores: ['node_modules/', 'dist/'],
    },
];
