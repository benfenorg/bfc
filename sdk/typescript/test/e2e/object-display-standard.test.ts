// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { beforeAll, describe, expect, it } from 'vitest';

import { BenfenObjectData } from '../../src/client';
import { bfc2HexAddress } from '../../src/utils/index.js';
import { publishPackage, setup, TestToolbox } from './utils/setup';

describe('Test Object Display Standard', () => {
	let toolbox: TestToolbox;
	let packageId: string;

	beforeAll(async () => {
		toolbox = await setup();
		const packagePath = __dirname + '/./data/display_test';
		({ packageId } = await publishPackage(packagePath, toolbox));
	});

	it('Test getting Display fields with error object', async () => {
		const resp = (
			await toolbox.client.getOwnedObjects({
				owner: toolbox.address(),
				options: { showDisplay: true, showType: true },
				filter: { StructType: `${packageId}::boars::Boar` },
			})
		).data;
		const data = resp[0].data as BenfenObjectData;
		const boarId = data.objectId;
		const display = (
			await toolbox.client.getObject({
				id: boarId,
				options: { showDisplay: true },
			})
		).data?.display!;
		const expectedData = {
			data: {
				age: '10',
				buyer: bfc2HexAddress(toolbox.address()),
				creator: 'Chris',
				description: `Unique Boar from the Boars collection with First Boar and ${bfc2HexAddress(
					boarId,
				)}`,
				img_url: 'https://get-a-boar.com/first.png',
				name: 'First Boar',
				price: '',
				project_url: 'https://get-a-boar.com/',
				full_url: 'https://get-a-boar.fullurl.com/',
				escape_syntax: '{name}',
			},
		};
		expect(display).toEqual(expect.objectContaining(expectedData));
		const errorMessage1 =
			'RPC call failed: Field value idd cannot be found in struct; RPC call failed: Field value namee cannot be found in struct';
		const errorMessage2 =
			'Field value idd cannot be found in struct; Field value namee cannot be found in struct';

		expect([errorMessage1, errorMessage2]).toContain((display.error as { error: string })?.error);
	});

	it('Test getting Display fields for object that has no display object', async () => {
		const coin = (await toolbox.getGasObjectsOwnedByAddress()).data[0];
		const coinId = coin.coinObjectId;
		const display = (
			await toolbox.client.getObject({
				id: coinId,
				options: { showDisplay: true },
			})
		).data?.display;
		expect(display?.data).toEqual(null);
	});
});
