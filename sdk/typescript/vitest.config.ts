// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { vanillaExtractPlugin } from '@vanilla-extract/vite-plugin';
import { configDefaults, defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [vanillaExtractPlugin()],
	test: {
		exclude: [...configDefaults.exclude, 'src/dapp-kit/**'],
		minThreads: 1,
		maxThreads: 8,
		hookTimeout: 1000000,
		testTimeout: 1000000,
		env: {
			NODE_ENV: 'test',
		},
	},
	resolve: {},
});
