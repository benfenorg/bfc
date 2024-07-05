// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

/// <reference types="vitest" />

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import { defineConfig } from 'vite';
import { configDefaults } from 'vitest/config';

export default defineConfig({
	root: __dirname,
	plugins: [vanillaExtractPlugin()],
	test: {
		include: ['./__tests__/hooks/**', './__tests__/components/**'],
		exclude: [...configDefaults.exclude, 'tests/**'],
		environment: 'jsdom',
		restoreMocks: true,
		globals: true,
		setupFiles: ['./__tests__/setup.ts'],
	},
	resolve: {
		alias: {},
	},
});
