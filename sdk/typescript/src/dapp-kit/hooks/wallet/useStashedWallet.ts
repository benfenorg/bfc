// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/hooks/wallet/useZkSendWallet.ts
========
import type { StashedWallet } from '@mysten/zksend';
import { registerStashedWallet } from '@mysten/zksend';
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/hooks/wallet/useStashedWallet.ts
import { useEffect, useLayoutEffect, useState } from 'react';

import type { ZkSendWallet } from '../../../zksend/index.js';
import { registerZkSendWallet } from '../../../zksend/index.js';
import { useAutoConnectWallet } from './useAutoConnectWallet.js';
import { useConnectWallet } from './useConnectWallet.js';

export interface StashedWalletConfig {
	name: string;
	origin?: string;
}

export function useStashedWallet(config?: StashedWalletConfig) {
	const status = useAutoConnectWallet();
	const [address, setAddress] = useState<string | null>(null);
	const [wallet, setWallet] = useState<StashedWallet | null>(null);
	const { mutate: connect } = useConnectWallet();

	useEffect(() => {
		// This handles an edge case where the user has already connected a wallet, but is coming from
		// a zkSend redirect, and we want to force the zkSend wallet to connect. We need to wait for the
		// autoconnection to attempt to connect, then force the zkSend wallet to connect.
		if (!address || !wallet || status !== 'attempted') return;

		connect({ wallet, silent: true });
		// Reset the address since we only want to do this once:
		setAddress(null);
	}, [address, status, connect, wallet]);

	useLayoutEffect(() => {
		if (!config?.name) {
			return;
		}

		const { wallet, unregister, addressFromRedirect } = registerStashedWallet(config.name, {
			origin: config.origin,
		});

		if (addressFromRedirect) {
			setWallet(wallet);
			setAddress(addressFromRedirect);
		}

		return unregister;
	}, [config?.name, config?.origin]);
}
