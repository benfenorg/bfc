// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from '../../../keypairs/ed25519/index.js';
import { ReadonlyWalletAccount } from '../../../wallet-standard/index.js';
import type { WalletAccount } from '../../../wallet-standard/index.js';

export function createMockAccount(accountOverrides: Partial<WalletAccount> = {}) {
	const keypair = new Ed25519Keypair();
	return new ReadonlyWalletAccount({
		address: keypair.getPublicKey().toSuiAddress(),
		publicKey: keypair.getPublicKey().toSuiBytes(),
		chains: ['sui:unknown'],
		features: ['sui:signAndExecuteTransactionBlock', 'sui:signTransactionBlock'],
		...accountOverrides,
	});
}
