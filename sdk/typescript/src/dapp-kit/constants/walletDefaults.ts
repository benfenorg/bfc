// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type {
	BenfenWalletFeatures,
	WalletWithRequiredFeatures,
} from '../../wallet-standard/index.js';
import { createInMemoryStore } from '../utils/stateStorage.js';

export const BENFEN_WALLET_NAME = 'Benfen Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'bfc-dapp-kit:wallet-connection-info';

const SIGN_FEATURES = [
	'bfc:signTransaction',
	'bfc:signTransactionBlock',
] satisfies (keyof BenfenWalletFeatures)[];

export const DEFAULT_WALLET_FILTER = (wallet: WalletWithRequiredFeatures) =>
	SIGN_FEATURES.some((feature) => wallet.features[feature]);

export const DEFAULT_PREFERRED_WALLETS = [BENFEN_WALLET_NAME];
