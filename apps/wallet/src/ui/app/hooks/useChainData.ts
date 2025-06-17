// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import useAppSelector from './useAppSelector';

export const useChainData = () => {
	const [activeApiEnv, activeRpcUrl] = useAppSelector(({ app }) => [app.apiEnv, app.customRPC]);

	return useMemo(() => {
		if (activeApiEnv === 'customRPC') {
			const url = new URL(activeRpcUrl as string);
			if (url.origin === 'https://devrpc4.openblock.vip') {
				return {
					ANONYMOUS_SWAP_POOL:
						'BFCf9a2b3794a667d5949f714b10c4c26967f9f74f01a0c5491445e1d4f7f2dbfb78aa1',
				};
			}
			return {
				ANONYMOUS_SWAP_POOL: '',
			};
		}
		return {
			ANONYMOUS_SWAP_POOL: '',
		};
	}, [activeApiEnv, activeRpcUrl]);
};
