// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SUI_FRAMEWORK_ADDRESS, SUI_SYSTEM_ADDRESS } from '@benfen/bfc.js';
=======
import { API_ENV } from '_src/shared/api-env';
>>>>>>> mainnet-v1.24.1
import { useFeatureValue } from '@growthbook/growthbook-react';

import useAppSelector from './useAppSelector';

const DEFAULT_RECOGNIZED_PACKAGES = [SUI_FRAMEWORK_ADDRESS, SUI_SYSTEM_ADDRESS];

export function useRecognizedPackages() {
	const apiEnv = useAppSelector((app) => app.app.apiEnv);
	const recognizedPackages = useFeatureValue('recognized-packages', DEFAULT_RECOGNIZED_PACKAGES);

	// Our recognized package list is currently only available on mainnet
	return apiEnv === API_ENV.mainnet ? recognizedPackages : DEFAULT_RECOGNIZED_PACKAGES;
}
