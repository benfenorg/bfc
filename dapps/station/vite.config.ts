// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import pluginRewriteAll from 'vite-plugin-rewrite-all';
import svgr from 'vite-plugin-svgr';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react(), svgr(), pluginRewriteAll()],
	resolve: {
		conditions: ['source'],
		alias: {
			'~': new URL('./src', import.meta.url).pathname,
		},
	},
});
