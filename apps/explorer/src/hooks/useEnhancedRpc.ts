// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { SuiClient } from '@benfen/bfc.js/client';
import { useMemo } from 'react';

import { useNetwork } from '~/context';
import { Network } from '~/utils/api/DefaultRpcClient';

// TODO: Use enhanced RPC locally by default
export function useEnhancedRpcClient() {
	const [network] = useNetwork();
	const client = useSuiClient();
	const enhancedRpc = useMemo(() => {
		if (network === Network.LOCAL) {
			return new SuiClient({ url: 'http://localhost:9124' });
		}

		return client;
	}, [network, client]);

	return enhancedRpc;
}
