// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/** The latest API version of the signTransactionBlock API. */
export type BfcSwitchChainVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type BfcSwitchChainFeature = {
	/** Namespace for the feature. */
	'bfc:switchChain': {
		/** Version of the feature API. */
		version: BfcSwitchChainVersion;
		switchChain: BfcSwitchChainMethod;
	};
};

export type BfcSwitchChainMethod = (input: BfcSwitchChainInput) => Promise<BfcSwitchChainOutput>;

/** Input for signing transactions. */
export interface BfcSwitchChainInput {
	chain: string;
}

/** Output of signing transactions. */
export type BfcSwitchChainOutput = string;
