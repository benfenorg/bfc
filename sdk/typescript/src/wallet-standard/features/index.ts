// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';
import type { SuiSignTransactionBlockFeature } from './suiSignTransactionBlock.js';
import type { SuiSignAndExecuteTransactionBlockFeature } from './suiSignAndExecuteTransactionBlock.js';
import type { SuiSignMessageFeature } from './suiSignMessage.js';
import type { SuiSignPersonalMessageFeature } from './suiSignPersonalMessage.js';
import type { BfcSwitchChainFeature } from './bfcSwitchChain.js';

/**
 * Wallet Standard features that are unique to Sui, and that all Sui wallets are expected to implement.
 */
export type SuiFeatures = SuiSignTransactionBlockFeature &
	SuiSignAndExecuteTransactionBlockFeature &
	SuiSignPersonalMessageFeature &
	BfcSwitchChainFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<SuiSignMessageFeature>;

export type WalletWithSuiFeatures = WalletWithFeatures<
	StandardConnectFeature &
		StandardEventsFeature &
		SuiFeatures &
		// Disconnect is an optional feature:
		Partial<StandardDisconnectFeature>
>;

export * from './suiSignMessage.js';
export * from './suiSignTransactionBlock.js';
export * from './suiSignAndExecuteTransactionBlock.js';
export * from './suiSignPersonalMessage.js';
export * from './bfcSwitchChain.js';
