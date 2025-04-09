// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { bcs, toBase64 } from '@benfen/bfc.js/bcs';
import {
	type BenfenClient,
	type BenfenTransactionBlockResponse,
	type BenfenTransactionBlockResponseOptions,
	type DryRunTransactionBlockResponse,
	type ExecuteTransactionRequestType,
} from '@benfen/bfc.js/client';
import { messageWithIntent } from '@benfen/bfc.js/cryptography';
import { isTransaction, type Transaction } from '@benfen/bfc.js/transactions';
import { fromB64, toB64 } from '@benfen/bfc.js/utils';
import { type SignedTransaction } from '@benfen/bfc.js/wallet-standard';

export type SignedMessage = {
	messageBytes: string;
	signature: string;
};

export abstract class WalletSigner {
	client: BenfenClient;

	constructor(client: BenfenClient) {
		this.client = client;
	}

	abstract signData(data: Uint8Array, clientIdentifier?: string): Promise<string>;

	abstract getAddress(): Promise<string>;

	async signMessage(input: { message: Uint8Array }): Promise<SignedMessage> {
		const signature = await this.signData(
			messageWithIntent('PersonalMessage', bcs.vector(bcs.u8()).serialize(input.message).toBytes()),
		);

		return {
			messageBytes: toB64(input.message),
			signature,
		};
	}

	protected async prepareTransactionBlock(transactionBlock: Uint8Array | Transaction | string) {
		if (isTransaction(transactionBlock)) {
			// If the sender has not yet been set on the transaction, then set it.
			// NOTE: This allows for signing transactions with mis-matched senders, which is important for sponsored transactions.
			transactionBlock.setSenderIfNotSet(await this.getAddress());
			return await transactionBlock.build({
				client: this.client,
			});
		}

		if (typeof transactionBlock === 'string') {
			return fromB64(transactionBlock);
		}

		if (transactionBlock instanceof Uint8Array) {
			return transactionBlock;
		}
		throw new Error('Unknown transaction format');
	}

	async signTransactionBlock(input: {
		transactionBlock: Uint8Array | Transaction;
	}): Promise<SignedTransaction> {
		const bytes = await this.prepareTransactionBlock(input.transactionBlock);
		const signature = await this.signData(messageWithIntent('TransactionData', bytes));

		return {
			bytes: toB64(bytes),
			signature,
		};
	}

	async signAndExecuteTransactionBlock(input: {
		transactionBlock: Uint8Array | Transaction;
		options?: BenfenTransactionBlockResponseOptions;
		requestType?: ExecuteTransactionRequestType;
	}): Promise<BenfenTransactionBlockResponse & SignedTransaction> {
		const bytes = await this.prepareTransactionBlock(input.transactionBlock);
		const signed = await this.signTransactionBlock({
			transactionBlock: bytes,
		});

		const result = await this.client.executeTransactionBlock({
			transactionBlock: bytes,
			signature: signed.signature,
			options: input.options,
			requestType: input.requestType,
		});

		return {
			...result,
			bytes: toBase64(bytes),
			signature: signed.signature,
		};
	}

	async dryRunTransactionBlock(input: {
		transactionBlock: Transaction | string | Uint8Array;
	}): Promise<DryRunTransactionBlockResponse> {
		return this.client.dryRunTransactionBlock({
			transactionBlock: await this.prepareTransactionBlock(input.transactionBlock),
		});
	}
}
