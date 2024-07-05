// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useEffect } from 'react';

import { getWallets } from '../../../wallet-standard/index.js';
import type { Wallet, WalletWithRequiredFeatures } from '../../../wallet-standard/index.js';
import { getRegisteredWallets } from '../../utils/walletUtils.js';
import { useWalletStore } from './useWalletStore.js';

/**
 * Internal hook for easily handling the addition and removal of new wallets.
 */
export function useWalletsChanged(
	preferredWallets: string[],
	requiredFeatures: (keyof WalletWithRequiredFeatures['features'])[],
) {
	const setWalletRegistered = useWalletStore((state) => state.setWalletRegistered);
	const setWalletUnregistered = useWalletStore((state) => state.setWalletUnregistered);

	useEffect(() => {
		const walletsApi = getWallets();
		let wallets: Wallet[] = [];

		const checkInterval = window.setInterval(() => {
			const registered = walletsApi.get();
			const newWallets = registered.filter((wallet) => !wallets.includes(wallet));
			if (newWallets.length > 0) {
				setWalletRegistered(getRegisteredWallets(preferredWallets, requiredFeatures));
				wallets = [...registered];
			}
		}, 100);

		const unsubscribeFromRegister = walletsApi.on('register', () => {
			setWalletRegistered(getRegisteredWallets(preferredWallets, requiredFeatures));
		});

		const unsubscribeFromUnregister = walletsApi.on('unregister', (unregisteredWallet) => {
			setWalletUnregistered(
				getRegisteredWallets(preferredWallets, requiredFeatures),
				unregisteredWallet,
			);
		});

		return () => {
			unsubscribeFromRegister();
			unsubscribeFromUnregister();
			clearInterval(checkInterval);
		};
	}, [preferredWallets, requiredFeatures, setWalletRegistered, setWalletUnregistered]);
}
