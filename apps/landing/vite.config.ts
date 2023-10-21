// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vite';
import path from 'path';
import handlebars from 'vite-plugin-handlebars';

process.env.VITE_VERCEL_ENV = process.env.VERCEL_ENV || 'development';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		handlebars({
			partialDirectory: path.resolve(__dirname, 'partials'),
		}) as Plugin,
	],
	build: {
		// Set the output directory to match what CRA uses:
		outDir: 'build',
		sourcemap: true,
		rollupOptions: {
			input: {
				main: path.resolve(__dirname, 'index.html'),
				agreement: path.resolve(__dirname, 'agreement.html'),
			},
		},
	},
});
