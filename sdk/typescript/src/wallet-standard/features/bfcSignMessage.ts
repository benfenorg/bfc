// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '@wallet-standard/core';

/**
 * The latest API version of the signMessage API.
 */
export type BenfenSignMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 */
export type BenfenSignMessageFeature = {
	/** Namespace for the feature. */
	'bfc:signMessage': {
		/** Version of the feature API. */
		version: BenfenSignMessageVersion;
		signMessage: BenfenSignMessageMethod;
	};
};

export type BenfenSignMessageMethod = (
	input: BenfenSignMessageInput,
) => Promise<BenfenSignMessageOutput>;

/**
 * Input for signing messages.
 */
export interface BenfenSignMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/**
 * Output of signing messages.
 */
export interface BenfenSignMessageOutput {
	messageBytes: string;
	signature: string;
}
