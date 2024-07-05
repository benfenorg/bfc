// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { beforeAll, describe, expect, it } from 'vitest';

import { BenfenObjectData } from '../../src/client/index.js';
import { bfc2HexAddress } from '../../src/utils/index.js';
import { publishPackage, setup, TestToolbox } from './utils/setup';

describe('Dynamic Fields Reading API', () => {
	let toolbox: TestToolbox;
	let packageId: string;
	let parentObjectId: string;

	beforeAll(async () => {
		toolbox = await setup();
		const packagePath = __dirname + '/./data/dynamic_fields';
		({ packageId } = await publishPackage(packagePath, toolbox));

		await toolbox.client
			.getOwnedObjects({
				owner: toolbox.address(),
				options: { showType: true },
				filter: { StructType: `${packageId}::dynamic_fields_test::Test` },
			})
			.then(function (objects) {
				const data = objects.data[0].data as BenfenObjectData;
				parentObjectId = data.objectId;
			});
	});

	it('get all dynamic fields', async () => {
		const dynamicFields = await toolbox.client.getDynamicFields({
			parentId: parentObjectId,
		});
		expect(dynamicFields.data.length).toEqual(2);
	});
	it('limit response in page', async () => {
		const dynamicFields = await toolbox.client.getDynamicFields({
			parentId: parentObjectId,
			limit: 1,
		});
		expect(dynamicFields.data.length).toEqual(1);
		expect(dynamicFields.nextCursor).not.toEqual(null);
	});
	it('go to next cursor', async () => {
		return await toolbox.client
			.getDynamicFields({ parentId: parentObjectId, limit: 1 })
			.then(async function (dynamicFields) {
				expect(dynamicFields.nextCursor).not.toEqual(null);

				const dynamicFieldsCursor = await toolbox.client.getDynamicFields({
					parentId: parentObjectId,
					cursor: dynamicFields.nextCursor,
				});
				expect(dynamicFieldsCursor.data.length).greaterThanOrEqual(0);
			});
	});
	it('get dynamic object field', async () => {
		const dynamicFields = await toolbox.client.getDynamicFields({
			parentId: parentObjectId,
		});
		for (const data of dynamicFields.data) {
			const objName = data.name;

			if (objName.type === '0x2::object::ID') {
				objName.value = bfc2HexAddress(objName.value as string);
			}

			const object = await toolbox.client.getDynamicFieldObject({
				parentId: bfc2HexAddress(parentObjectId),
				name: objName,
			});

			expect(object.data?.objectId).toEqual(data.objectId);
		}
	});
});
