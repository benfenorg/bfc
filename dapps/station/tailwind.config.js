// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import colors from 'tailwindcss/colors';

/** @type {import('tailwindcss').Config} */
const config = {
	content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
	theme: {
		screens: {
			sm: '480px',
			md: '768px',
			lg: '976px',
			xl: '1440px',
		},
		colors: {
			white: colors.white,
			black: colors.black,
			bf: {
				DEFAULT: '#171719',
				white: '#ffffff',
				white_4p: '#ffffff0a',
				orange: '#E18416',
				orange_10p: '#E184161A',
				green: '#32BA89',
				green_10p: '#32BA891A',
				red: '#EB362A',
				red_10p: '#EB362A1A',
				link: '#22367B',
				hover: '#FFFFFF14',
				text1: '#171719',
				text2: '#5A6070',
				text3: '#A3A8B5',
				border: '#E1E1E9',
				card: '#F8F8FA',
				press: '#FFFFFF24',
			},
		},
		extend: {
			container: {
				center: true,
				padding: '1rem',
			},
			colors: {
				primary: '#101827',
			},
		},
	},
	plugins: [require('@headlessui/tailwindcss')],
};

export default config;
