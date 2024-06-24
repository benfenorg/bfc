// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { WalletWithRequiredFeatures } from '../../wallet-standard/index.js';
import { createInMemoryStore } from '../utils/stateStorage.js';

export const SUI_WALLET_NAME = 'Benfen Wallet';

export const DEFAULT_STORAGE =
	typeof window !== 'undefined' && window.localStorage ? localStorage : createInMemoryStore();

export const DEFAULT_STORAGE_KEY = 'bfc-dapp-kit:wallet-connection-info';

export const DEFAULT_REQUIRED_FEATURES: (keyof WalletWithRequiredFeatures['features'])[] = [
	'bfc:signTransactionBlock',
];

export const DEFAULT_PREFERRED_WALLETS = [SUI_WALLET_NAME];
