// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const postcssPresetEnv = require('postcss-preset-env');
const tailwind = require('tailwindcss');

module.exports = {
    plugins: [require('postcss-preset-env'), require('tailwindcss')],
};
