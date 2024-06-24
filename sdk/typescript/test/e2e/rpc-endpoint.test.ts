// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { beforeAll, describe, expect, it } from 'vitest';

import { SuiGasData } from '../../src/client';
import { setup, TestToolbox } from './utils/setup';

describe('Invoke any RPC endpoint', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('bfcx_getOwnedObjects', async () => {
		const gasObjectsExpected = await toolbox.client.getOwnedObjects({
			owner: toolbox.address(),
		});
		const gasObjects = await toolbox.client.call<{ data: SuiGasData }>('bfcx_getOwnedObjects', [
			toolbox.address(),
		]);
		expect(gasObjects.data).toStrictEqual(gasObjectsExpected.data);
	});

	it('bfc_getObjectOwnedByAddress Error', async () => {
		expect(toolbox.client.call('bfcx_getOwnedObjects', [])).rejects.toThrowError();
	});

	it('bfcx_getCommitteeInfo', async () => {
		const committeeInfoExpected = await toolbox.client.getCommitteeInfo();

		const committeeInfo = await toolbox.client.call('bfcx_getCommitteeInfo', []);

		expect(committeeInfo).toStrictEqual(committeeInfoExpected);
	});
});
