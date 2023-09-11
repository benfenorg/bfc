// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import preset from '@mysten/core/tailwind.config';
import { type Config } from 'tailwindcss';

export default {
	content: [
		'./index.html',
		'./agreement.html',
		'./partials/**/*.html',
		'./src/**/*.{js,jsx,ts,tsx}',
		'./node_modules/@mysten/ui/src/**/*.{js,jsx,ts,tsx}',
	],
	presets: [preset],
} satisfies Partial<Config>;
