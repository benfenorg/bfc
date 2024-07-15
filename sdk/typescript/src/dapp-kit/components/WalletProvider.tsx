// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { ReactNode } from 'react';
import { useRef } from 'react';
import type { StateStorage } from 'zustand/middleware';

import type {
	WalletWithFeatures,
	WalletWithRequiredFeatures,
} from '../../wallet-standard/index.js';
import {
	DEFAULT_PREFERRED_WALLETS,
	DEFAULT_REQUIRED_FEATURES,
	DEFAULT_STORAGE,
	DEFAULT_STORAGE_KEY,
} from '../constants/walletDefaults.js';
import { WalletContext } from '../contexts/walletContext.js';
import { useAutoConnectWallet } from '../hooks/wallet/useAutoConnectWallet.js';
import { useWalletPropertiesChanged } from '../hooks/wallet/useWalletPropertiesChanged.js';
import { useWalletsChanged } from '../hooks/wallet/useWalletsChanged.js';
import { createInMemoryStore } from '../utils/stateStorage.js';
import { getRegisteredWallets } from '../utils/walletUtils.js';
import { createWalletStore } from '../walletStore.js';

export type WalletProviderProps = {
	/** A list of wallets that are sorted to the top of the wallet list, if they are available to connect to. By default, wallets are sorted by the order they are loaded in. */
	preferredWallets?: string[];

	/** A list of features that are required for the dApp to function. This filters the list of wallets presented to users when selecting a wallet to connect from, ensuring that only wallets that meet the dApps requirements can connect. */
	requiredFeatures?: (keyof WalletWithRequiredFeatures['features'])[];

	/** Enables automatically reconnecting to the most recently used wallet account upon mounting. */
	autoConnect?: boolean;

	/** Configures how the most recently connected to wallet account is stored. Set to `null` to disable persisting state entirely. Defaults to using localStorage if it is available. */
	storage?: StateStorage | null;

	/** The key to use to store the most recently connected wallet account. */
	storageKey?: string;

	children: ReactNode;
};

export type { WalletWithFeatures };

export function WalletProvider({
	preferredWallets = DEFAULT_PREFERRED_WALLETS,
	requiredFeatures = DEFAULT_REQUIRED_FEATURES,
	storage = DEFAULT_STORAGE,
	storageKey = DEFAULT_STORAGE_KEY,
	autoConnect = false,
	children,
}: WalletProviderProps) {
	const storeRef = useRef(
		createWalletStore({
			autoConnectEnabled: autoConnect,
			wallets: getRegisteredWallets(preferredWallets, requiredFeatures),
			storage: storage || createInMemoryStore(),
			storageKey,
		}),
	);

	return (
		<WalletContext.Provider value={storeRef.current}>
			<WalletConnectionManager
				preferredWallets={preferredWallets}
				requiredFeatures={requiredFeatures}
			>
				{children}
			</WalletConnectionManager>
		</WalletContext.Provider>
	);
}

type WalletConnectionManagerProps = Pick<
	WalletProviderProps,
	'preferredWallets' | 'requiredFeatures' | 'children'
>;

function WalletConnectionManager({
	preferredWallets = DEFAULT_PREFERRED_WALLETS,
	requiredFeatures = DEFAULT_REQUIRED_FEATURES,
	children,
}: WalletConnectionManagerProps) {
	useWalletsChanged(preferredWallets, requiredFeatures);
	useWalletPropertiesChanged();
	useAutoConnectWallet();

	return children;
}
