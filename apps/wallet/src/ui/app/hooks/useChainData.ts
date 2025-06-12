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
					ANONYMOUS_SWAP_POOL: '0x340d599ffa3137e87b26c30117632dd95298840e9ab3f1c60bd271db19c39c8b',
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
