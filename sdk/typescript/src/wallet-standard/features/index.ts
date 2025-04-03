// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';

import type { BenfenReportTransactionEffectsFeature } from './bfcReportTransactionEffects.js';
import type { BenfenSignAndExecuteTransactionFeature } from './bfcSignAndExecuteTransaction.js';
import type { BenfenSignAndExecuteTransactionBlockFeature } from './bfcSignAndExecuteTransactionBlock.js';
import type { BenfenSignMessageFeature } from './bfcSignMessage.js';
import type { BenfenSignPersonalMessageFeature } from './bfcSignPersonalMessage.js';
import type { BenfenSignTransactionFeature } from './bfcSignTransaction.js';
import type { BenfenSignTransactionBlockFeature } from './bfcSignTransactionBlock.js';
import type { BfcSwitchChainFeature } from './bfcSwitchChain.js';

/**
 * Wallet Standard features that are unique to Benfen, and that all Benfen wallets are expected to implement.
 */
export type BenfenFeatures = Partial<BenfenSignTransactionBlockFeature> &
	Partial<BenfenSignAndExecuteTransactionBlockFeature> &
	BenfenSignPersonalMessageFeature &
	BenfenSignAndExecuteTransactionFeature &
	BenfenSignTransactionFeature &
	BfcSwitchChainFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<BenfenSignMessageFeature> &
	Partial<BenfenReportTransactionEffectsFeature>;

export type BenfenWalletFeatures = StandardConnectFeature &
	StandardEventsFeature &
	BenfenFeatures &
	// Disconnect is an optional feature:
	Partial<StandardDisconnectFeature>;

export type WalletWithBenfenFeatures = WalletWithFeatures<BenfenWalletFeatures>;

/**
 * Represents a wallet with the absolute minimum feature set required to function in the Benfen ecosystem.
 */
export type WalletWithRequiredFeatures = WalletWithFeatures<
	MinimallyRequiredFeatures &
		Partial<BenfenFeatures> &
		Partial<StandardDisconnectFeature> &
		IdentifierRecord<unknown>
>;

export type MinimallyRequiredFeatures = StandardConnectFeature & StandardEventsFeature;

export * from './bfcSignMessage.js';
export * from './bfcSignTransactionBlock.js';
export * from './bfcSignTransaction.js';
export * from './bfcSignAndExecuteTransactionBlock.js';
export * from './bfcSignAndExecuteTransaction.js';
export * from './bfcSignPersonalMessage.js';
export * from './bfcReportTransactionEffects.js';
export * from './bfcSwitchChain.js';
