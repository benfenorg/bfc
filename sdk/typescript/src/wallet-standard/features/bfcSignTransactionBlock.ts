// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

import type { TransactionBlock } from '../../transactions/index.js';

/** The latest API version of the signTransactionBlock API. */
export type BenfenSignTransactionBlockVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type BenfenSignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'bfc:signTransactionBlock': {
		/** Version of the feature API. */
		version: BenfenSignTransactionBlockVersion;
		signTransactionBlock: BenfenSignTransactionBlockMethod;
	};
};

export type BenfenSignTransactionBlockMethod = (
	input: BenfenSignTransactionBlockInput,
) => Promise<BenfenSignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface BenfenSignTransactionBlockInput {
	transactionBlock: TransactionBlock;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface BenfenSignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	transactionBlockBytes: string;
	signature: string;
}
