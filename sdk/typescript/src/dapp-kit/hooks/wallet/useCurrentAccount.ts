// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '../../../wallet-standard/index.js';
import { useWalletStore } from './useWalletStore.js';

/**
 * Retrieves the wallet account that is currently selected, if one exists.
 */
export function useCurrentAccount(): WalletAccount | null {
	return useWalletStore((state) => state.currentAccount);
}
