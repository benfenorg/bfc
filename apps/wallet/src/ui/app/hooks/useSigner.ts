// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type SerializedUIAccount } from '_src/background/accounts/Account';

import { walletApiProvider } from '../ApiProvider';
import { type WalletSigner } from '../WalletSigner';
import { useBackgroundClient } from './useBackgroundClient';

export function useSigner(account: SerializedUIAccount | null): WalletSigner | null {
	const background = useBackgroundClient();
	if (!account) {
		return null;
	}
	return walletApiProvider.getSignerInstance(account, background);
}
