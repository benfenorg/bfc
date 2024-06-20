// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '../../../wallet-standard/index.js';
import { useWalletStore } from './useWalletStore.js';

/**
 * Retrieves a list of connected accounts authorized by the dApp.
 */
export function useAccounts(): readonly WalletAccount[] {
	return useWalletStore((state) => state.accounts);
}
