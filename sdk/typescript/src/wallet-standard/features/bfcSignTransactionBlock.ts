// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

import type { Transaction } from '../../transactions/index.js';

/** The latest API version of the signTransactionBlock API. */
export type BenfenSignTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `bfc:signTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type BenfenSignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'bfc:signTransactionBlock': {
		/** Version of the feature API. */
		version: BenfenSignTransactionBlockVersion;
		/** @deprecated Use `bfc:signTransaction` instead. */
		signTransactionBlock: BenfenSignTransactionBlockMethod;
	};
};

/** @deprecated Use `bfc:signTransaction` instead. */
export type BenfenSignTransactionBlockMethod = (
	input: BenfenSignTransactionBlockInput,
) => Promise<BenfenSignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface BenfenSignTransactionBlockInput {
	transactionBlock: Transaction;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface BenfenSignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	/** Transaction as base64 encoded bcs. */
	transactionBlockBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
