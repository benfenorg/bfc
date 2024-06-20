// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, it } from 'vitest';

import { Coin, sui2BfcAddress } from '../../src';
import { setup } from './utils/setup';

describe('Coin related API', () => {
	it('test Coin utility functions', async () => {
		const toolbox = await setup();
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		coins.forEach((c) => {
			expect(Coin.isCoin(c)).toBeTruthy();
			expect(Coin.isBFC(c)).toBeTruthy();
		});
	});

	it('test getCoinStructTag', async () => {
		const toolbox = await setup();
		const exampleStructTag = {
			address: sui2BfcAddress('0x2'),
			module: 'bfc',
			name: 'BFC',
			typeParams: [],
		};
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		const coinTypeArg: string = Coin.getCoinTypeArg(coins[0])!;
		expect(Coin.getCoinStructTag(coinTypeArg)).toStrictEqual(exampleStructTag);
	});
});
