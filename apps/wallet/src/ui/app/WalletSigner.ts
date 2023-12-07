// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SignedMessage, type SignedTransaction, SignerWithProvider } from '@benfen/bfc.js';
import {
	type ExecuteTransactionRequestType,
	type SuiTransactionBlockResponse,
	type SuiTransactionBlockResponseOptions,
} from '@benfen/bfc.js/client';
import { type SerializedSignature } from '@benfen/bfc.js/cryptography';
import { type TransactionBlock } from '@benfen/bfc.js/transactions';

export abstract class WalletSigner extends SignerWithProvider {
	abstract signData(data: Uint8Array, clientIdentifier?: string): Promise<SerializedSignature>;

	async signMessage(
		input: { message: Uint8Array },
		clientIdentifier?: string,
	): Promise<SignedMessage> {
		return super.signMessage(input);
	}
	async signTransactionBlock(
		input: {
			transactionBlock: Uint8Array | TransactionBlock;
		},
		clientIdentifier?: string,
	): Promise<SignedTransaction> {
		return super.signTransactionBlock(input);
	}
	async signAndExecuteTransactionBlock(
		input: {
			transactionBlock: Uint8Array | TransactionBlock;
			options?: SuiTransactionBlockResponseOptions;
			requestType?: ExecuteTransactionRequestType;
		},
		clientIdentifier?: string,
	): Promise<SuiTransactionBlockResponse> {
		return super.signAndExecuteTransactionBlock(input);
	}
}
