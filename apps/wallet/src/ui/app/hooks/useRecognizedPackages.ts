// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { BENFEN_FRAMEWORK_ADDRESS, BENFEN_SYSTEM_ADDRESS } from '@benfen/bfc.js/utils';

const DEFAULT_RECOGNIZED_PACKAGES = [BENFEN_FRAMEWORK_ADDRESS, BENFEN_SYSTEM_ADDRESS];

export function useRecognizedPackages() {
	// Our recognized package list is currently only available on mainnet
	return DEFAULT_RECOGNIZED_PACKAGES;
}
