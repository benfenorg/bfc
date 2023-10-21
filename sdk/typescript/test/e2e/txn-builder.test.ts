// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll, beforeEach } from 'vitest';

import {
	SuiTransactionBlockResponse,
	SuiObjectData,
	SuiObjectChangeCreated,
} from '../../src/client';
import { normalizeSuiObjectId } from '../../src/utils';
import { SUI_SYSTEM_STATE_OBJECT_ID } from '../../src/framework';
import type { Keypair } from '../../src/cryptography';
import {
	DEFAULT_RECIPIENT,
	DEFAULT_GAS_BUDGET,
	setup,
	TestToolbox,
	publishPackage,
	upgradePackage,
} from './utils/setup';
import { SuiClient } from '../../src/client';
import {} from '../../src/builder/TransactionBlockData';
import { TransactionBlock } from '../../src/builder';

export const SUI_CLOCK_OBJECT_ID = normalizeSuiObjectId('0x6');

describe('Transaction Builders', () => {
	let toolbox: TestToolbox;
	let packageId: string;
	let publishTxn: SuiTransactionBlockResponse;
	let sharedObjectId: string;

	beforeAll(async () => {
		const packagePath = __dirname + '/./data/serializer';
		({ packageId, publishTxn } = await publishPackage(packagePath));
		const sharedObject = (publishTxn.effects?.created)!.filter(
			(o) =>
				typeof o.owner === 'object' &&
				'Shared' in o.owner &&
				o.owner.Shared.initial_shared_version !== undefined,
		)[0];
		sharedObjectId = sharedObject.reference.objectId;
	});

	beforeEach(async () => {
		toolbox = await setup();
	});

	it('SplitCoins + TransferObjects', async () => {
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		const tx = new TransactionBlock();
		const coin_0 = coins[0].data as SuiObjectData;

		const coin = tx.splitCoins(tx.object(coin_0.objectId), [tx.pure(DEFAULT_GAS_BUDGET * 2)]);
		tx.transferObjects([coin], tx.pure(toolbox.address()));
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('MergeCoins', async () => {
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		const coin_0 = coins[0].data as SuiObjectData;
		const coin_1 = coins[1].data as SuiObjectData;
		const tx = new TransactionBlock();
		tx.mergeCoins(tx.object(coin_0.objectId), [tx.object(coin_1.objectId)]);
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('MoveCall', async () => {
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		const coin_0 = coins[0].data as SuiObjectData;
		const tx = new TransactionBlock();
		tx.moveCall({
			target: '0x2::pay::split',
			typeArguments: ['0x2::bfc::BFC'],
			arguments: [tx.object(coin_0.objectId), tx.pure(DEFAULT_GAS_BUDGET * 2)],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it(
		'MoveCall Shared Object',
		async () => {
			const coins = await toolbox.getGasObjectsOwnedByAddress();
			const coin_2 = coins[2].data as SuiObjectData;

			const [{ suiAddress: validatorAddress }] = await toolbox.getActiveValidators();

			const tx = new TransactionBlock();
			tx.moveCall({
				target: '0x3::sui_system::request_add_stake',
				arguments: [
					tx.object(SUI_SYSTEM_STATE_OBJECT_ID),
					tx.object(coin_2.objectId),
					tx.pure(validatorAddress),
				],
			});

			await validateTransaction(toolbox.client, toolbox.keypair, tx);
		},
		{
			// TODO: This test is currently flaky, so adding a retry to unblock merging
			retry: 10,
		},
	);

	it('SplitCoins from gas object + TransferObjects', async () => {
		const tx = new TransactionBlock();
		const coin = tx.splitCoins(tx.gas, [tx.pure(1)]);
		tx.transferObjects([coin], tx.pure(DEFAULT_RECIPIENT));
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('TransferObjects gas object', async () => {
		const tx = new TransactionBlock();
		tx.transferObjects([tx.gas], tx.pure(DEFAULT_RECIPIENT));
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('TransferObject', async () => {
		const coins = await toolbox.getGasObjectsOwnedByAddress();
		const tx = new TransactionBlock();
		const coin_0 = coins[2].data as SuiObjectData;

		tx.transferObjects([tx.object(coin_0.objectId)], tx.pure(DEFAULT_RECIPIENT));
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('Move Shared Object Call with mixed usage of mutable and immutable references', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::serializer_tests::value`,
			arguments: [tx.object(sharedObjectId)],
		});
		tx.moveCall({
			target: `${packageId}::serializer_tests::set_value`,
			arguments: [tx.object(sharedObjectId)],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it('immutable clock', async () => {
		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::serializer_tests::use_clock`,
			arguments: [tx.object(SUI_CLOCK_OBJECT_ID)],
		});
		await validateTransaction(toolbox.client, toolbox.keypair, tx);
	});

	it(
		'Publish and Upgrade Package',
		async () => {
			// Step 1. Publish the package
			const originalPackagePath = __dirname + '/./data/serializer';
			const { packageId, publishTxn } = await publishPackage(originalPackagePath, toolbox);

			const capId = (
				publishTxn.objectChanges?.find(
					(a) =>
						a.type === 'created' &&
						a.objectType.endsWith('UpgradeCap') &&
						'Immutable' !== a.owner &&
						'AddressOwner' in a.owner &&
						a.owner.AddressOwner === toolbox.address(),
				) as SuiObjectChangeCreated
			)?.objectId;

			expect(capId).toBeTruthy();

			const sharedObjectId = (publishTxn.effects?.created)!.filter(
				(o) =>
					typeof o.owner === 'object' &&
					'Shared' in o.owner &&
					o.owner.Shared.initial_shared_version !== undefined,
			)[0].reference.objectId;

			// Step 2. Confirm that its functions work as expected in its
			// first version
			let callOrigTx = new TransactionBlock();
			callOrigTx.moveCall({
				target: `${packageId}::serializer_tests::value`,
				arguments: [callOrigTx.object(sharedObjectId)],
			});
			callOrigTx.moveCall({
				target: `${packageId}::serializer_tests::set_value`,
				arguments: [callOrigTx.object(sharedObjectId)],
			});
			await validateTransaction(toolbox.client, toolbox.keypair, callOrigTx);

			// Step 3. Publish the upgrade for the package.
			const upgradedPackagePath = __dirname + '/./data/serializer_upgrade';

			// Step 4. Make sure the behaviour of the upgrade package matches
			// the newly introduced function
			await upgradePackage(packageId, capId, upgradedPackagePath, toolbox);
		},
		{
			// TODO: This test is currently flaky, so adding a retry to unblock merging
			retry: 10,
		},
	);
});

async function validateTransaction(client: SuiClient, signer: Keypair, tx: TransactionBlock) {
	tx.setSenderIfNotSet(signer.getPublicKey().toSuiAddress());
	const localDigest = await tx.getDigest({ client });
	const result = await client.signAndExecuteTransactionBlock({
		signer,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
	expect(localDigest).toEqual(result.digest);
	expect(result.effects?.status.status).toEqual('success');
}
