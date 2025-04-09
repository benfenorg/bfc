// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/**
 * A Wallet Standard feature for reporting the effects of a transaction block executed by a dapp
 * The feature allows wallets to updated their caches using the effects of the transaction
 * executed outside of the wallet
 */
export type BenfenReportTransactionEffectsFeature = {
	/** Namespace for the feature. */
	'bfc:reportTransactionEffects': {
		/** Version of the feature API. */
		version: '1.0.0';
		reportTransactionEffects: BenfenReportTransactionEffectsMethod;
	};
};

export type BenfenReportTransactionEffectsMethod = (
	input: BenfenReportTransactionEffectsInput,
) => Promise<void>;

/** Input for signing transactions. */
export interface BenfenReportTransactionEffectsInput {
	account: WalletAccount;
	chain: IdentifierString;
	/** Transaction effects as base64 encoded bcs. */
	effects: string;
}
