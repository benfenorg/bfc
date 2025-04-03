// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { WalletWithFeatures } from '@wallet-standard/core';

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import { fromBase64, toBase64 } from '../utils/index.js';
import type {
	BenfenSignAndExecuteTransactionInput,
	BenfenSignTransactionInput,
	BenfenWalletFeatures,
} from './features/index.js';

declare module '@wallet-standard/core' {
	export interface Wallet {
		/**
		 * Unique identifier of the Wallet.
		 *
		 * If not provided, the wallet name will be used as the identifier.
		 */
		readonly id?: string;
	}

	export interface StandardConnectOutput {
		supportedIntents?: string[];
	}
}

export type { Wallet } from '@wallet-standard/core';

export async function signAndExecuteTransaction(
	wallet: WalletWithFeatures<Partial<BenfenWalletFeatures>>,
	input: BenfenSignAndExecuteTransactionInput,
) {
	if (wallet.features['bfc:signAndExecuteTransaction']) {
		return wallet.features['bfc:signAndExecuteTransaction'].signAndExecuteTransaction(input);
	}

	if (!wallet.features['bfc:signAndExecuteTransactionBlock']) {
		throw new Error(
			`Provided wallet (${wallet.name}) does not support the signAndExecuteTransaction feature.`,
		);
	}

	const { signAndExecuteTransactionBlock } = wallet.features['bfc:signAndExecuteTransactionBlock'];

	const transactionBlock = Transaction.from(await input.transaction.toJSON());
	const { digest, rawEffects, rawTransaction } = await signAndExecuteTransactionBlock({
		account: input.account,
		chain: input.chain,
		transactionBlock,
		options: {
			showRawEffects: true,
			showRawInput: true,
		},
	});

	const [
		{
			txSignatures: [signature],
			intentMessage: { value: bcsTransaction },
		},
	] = bcs.SenderSignedData.parse(fromBase64(rawTransaction!));

	const bytes = bcs.TransactionData.serialize(bcsTransaction).toBase64();

	return {
		digest,
		signature,
		bytes,
		effects: toBase64(new Uint8Array(rawEffects!)),
	};
}

export async function signTransaction(
	wallet: WalletWithFeatures<Partial<BenfenWalletFeatures>>,
	input: BenfenSignTransactionInput,
) {
	if (wallet.features['bfc:signTransaction']) {
		return wallet.features['bfc:signTransaction'].signTransaction(input);
	}

	if (!wallet.features['bfc:signTransactionBlock']) {
		throw new Error(
			`Provided wallet (${wallet.name}) does not support the signTransaction feature.`,
		);
	}

	const { signTransactionBlock } = wallet.features['bfc:signTransactionBlock'];

	const transaction = Transaction.from(await input.transaction.toJSON());
	const { transactionBlockBytes, signature } = await signTransactionBlock({
		transactionBlock: transaction,
		account: input.account,
		chain: input.chain,
	});

	return { bytes: transactionBlockBytes, signature };
}
