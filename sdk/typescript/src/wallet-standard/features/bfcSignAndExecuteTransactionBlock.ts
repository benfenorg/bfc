// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type {
	BenfenTransactionBlockResponse,
	BenfenTransactionBlockResponseOptions,
	ExecuteTransactionRequestType,
} from '../../client/index.js';
import type { BenfenSignTransactionBlockInput } from './bfcSignTransactionBlock.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type BenfenSignAndExecuteTransactionBlockVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type BenfenSignAndExecuteTransactionBlockFeature = {
	/** Namespace for the feature. */
	'bfc:signAndExecuteTransactionBlock': {
		/** Version of the feature API. */
		version: BenfenSignAndExecuteTransactionBlockVersion;
		signAndExecuteTransactionBlock: BenfenSignAndExecuteTransactionBlockMethod;
	};
};

export type BenfenSignAndExecuteTransactionBlockMethod = (
	input: BenfenSignAndExecuteTransactionBlockInput,
) => Promise<BenfenSignAndExecuteTransactionBlockOutput>;

/** Input for signing and sending transactions. */
export interface BenfenSignAndExecuteTransactionBlockInput extends BenfenSignTransactionBlockInput {
	/**
	 * `WaitForEffectsCert` or `WaitForLocalExecution`, see details in `ExecuteTransactionRequestType`.
	 * Defaults to `WaitForLocalExecution` if options.showEffects or options.showEvents is true
	 */
	requestType?: ExecuteTransactionRequestType;
	/** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
	options?: BenfenTransactionBlockResponseOptions;
}

/** Output of signing and sending transactions. */
export interface BenfenSignAndExecuteTransactionBlockOutput
	extends BenfenTransactionBlockResponse {}
