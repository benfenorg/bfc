// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { BenfenSignTransactionInput, SignedTransaction } from './bfcSignTransaction.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type BenfenSignAndExecuteTransactionVersion = '2.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type BenfenSignAndExecuteTransactionFeature = {
	/** Namespace for the feature. */
	'bfc:signAndExecuteTransaction': {
		/** Version of the feature API. */
		version: BenfenSignAndExecuteTransactionVersion;
		signAndExecuteTransaction: BenfenSignAndExecuteTransactionMethod;
	};
};

export type BenfenSignAndExecuteTransactionMethod = (
	input: BenfenSignAndExecuteTransactionInput,
) => Promise<BenfenSignAndExecuteTransactionOutput>;

/** Input for signing and sending transactions. */
export interface BenfenSignAndExecuteTransactionInput extends BenfenSignTransactionInput {}

/** Output of signing and sending transactions. */
export interface BenfenSignAndExecuteTransactionOutput extends SignedTransaction {
	digest: string;
	/** Transaction effects as base64 encoded bcs. */
	effects: string;
}
