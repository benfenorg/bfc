// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SharedObjectRef } from '@benfen/bfc.js/bcs';
import { SuiObjectRef } from '@benfen/bfc.js/client';
import { TransactionArgument } from '@benfen/bfc.js/transactions';
=======
import type { SuiClient } from '@mysten/sui.js/client';
import type { TransactionObjectArgument } from '@mysten/sui.js/transactions';
>>>>>>> mainnet-v1.24.1

import type { BaseRulePackageIds } from '../constants.js';

export * from './kiosk.js';
export * from './transfer-policy.js';

/**
 * A valid argument for any of the Kiosk functions.
 */
export type ObjectArgument = string | TransactionObjectArgument;

/**
 * A Network selector.
 * Kiosk SDK supports mainnet & testnet.
 * Pass `custom` for any other network (devnet, localnet).
 */
export enum Network {
	MAINNET = 'mainnet',
	TESTNET = 'testnet',
	CUSTOM = 'custom',
}

/**
 * The Client Options for Both KioskClient & TransferPolicyManager.
 */
export type KioskClientOptions = {
	client: SuiClient;
	network: Network;
	packageIds?: BaseRulePackageIds;
};
