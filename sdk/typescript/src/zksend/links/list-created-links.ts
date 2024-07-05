// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { BenfenClient } from '../../client/index.js';
import type { ZkBagContractOptions } from './zk-bag.js';
import { MAINNET_CONTRACT_IDS } from './zk-bag.js';

export async function listCreatedLinks({
	address,
	cursor,
	network,
	contract = MAINNET_CONTRACT_IDS,
	fetch: fetchFn,
	...linkOptions
}: {
	address: string;
	contract?: ZkBagContractOptions;
	cursor?: string;
	network?: 'mainnet' | 'testnet';

	// Link options:
	host?: string;
	path?: string;
	client?: BenfenClient;
	fetch?: typeof fetch;
}) {
	throw new Error('Failed to load created links');
}
