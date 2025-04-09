// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { WalletAccount } from '@wallet-standard/core';

/**
 * The latest API version of the signMessage API.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `bfc:signPersonalMessage` feature
 */
export type BenfenSignMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 *
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `bfc:signPersonalMessage` feature
 */
export type BenfenSignMessageFeature = {
	/** Namespace for the feature. */
	'bfc:signMessage': {
		/** Version of the feature API. */
		version: BenfenSignMessageVersion;
		signMessage: BenfenSignMessageMethod;
	};
};

/** @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `bfc:signPersonalMessage` feature */
export type BenfenSignMessageMethod = (
	input: BenfenSignMessageInput,
) => Promise<BenfenSignMessageOutput>;

/**
 * Input for signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `bfc:signPersonalMessage` feature
 */
export interface BenfenSignMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/**
 * Output of signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `bfc:signPersonalMessage` feature
 */
export interface BenfenSignMessageOutput {
	/** Base64 message bytes. */
	messageBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
