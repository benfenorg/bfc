// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';
<<<<<<< HEAD:sdk/typescript/src/wallet-standard/features/index.ts
import type { SuiSignTransactionBlockFeature } from './suiSignTransactionBlock.js';
import type { SuiSignAndExecuteTransactionBlockFeature } from './suiSignAndExecuteTransactionBlock.js';
import type { SuiSignMessageFeature } from './suiSignMessage.js';
import type { SuiSignPersonalMessageFeature } from './suiSignPersonalMessage.js';
=======

import type { SuiSignAndExecuteTransactionBlockFeature } from './suiSignAndExecuteTransactionBlock.js';
import type { SuiSignMessageFeature } from './suiSignMessage.js';
import type { SuiSignPersonalMessageFeature } from './suiSignPersonalMessage.js';
import type { SuiSignTransactionBlockFeature } from './suiSignTransactionBlock.js';
>>>>>>> mainnet-v1.24.1:sdk/wallet-standard/src/features/index.ts

/**
 * Wallet Standard features that are unique to Sui, and that all Sui wallets are expected to implement.
 */
export type SuiFeatures = SuiSignTransactionBlockFeature &
	SuiSignAndExecuteTransactionBlockFeature &
	SuiSignPersonalMessageFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<SuiSignMessageFeature>;

export type WalletWithSuiFeatures = WalletWithFeatures<
	StandardConnectFeature &
		StandardEventsFeature &
		SuiFeatures &
		// Disconnect is an optional feature:
		Partial<StandardDisconnectFeature>
>;

<<<<<<< HEAD:sdk/typescript/src/wallet-standard/features/index.ts
=======
/**
 * Represents a wallet with the absolute minimum feature set required to function in the Sui ecosystem.
 */
export type WalletWithRequiredFeatures = WalletWithFeatures<
	MinimallyRequiredFeatures &
		Partial<SuiFeatures> &
		Partial<StandardDisconnectFeature> &
		IdentifierRecord<unknown>
>;

export type MinimallyRequiredFeatures = StandardConnectFeature & StandardEventsFeature;

>>>>>>> mainnet-v1.24.1:sdk/wallet-standard/src/features/index.ts
export * from './suiSignMessage.js';
export * from './suiSignTransactionBlock.js';
export * from './suiSignAndExecuteTransactionBlock.js';
export * from './suiSignPersonalMessage.js';
