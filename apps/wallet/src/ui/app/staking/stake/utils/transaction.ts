// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { TransactionBlock } from '@benfen/bfc.js/transactions';
import { BFC_SYSTEM_STATE_OBJECT_ID } from '@benfen/bfc.js/utils';

export function createStakeTransaction(amount: bigint, validator: string) {
	const tx = new TransactionBlock();
	const stakeCoin = tx.splitCoins(tx.gas, [amount]);
	tx.moveCall({
		target: '0x3::sui_system::request_add_stake',
		arguments: [
			tx.sharedObjectRef({
				objectId: BFC_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			}),
			stakeCoin,
			tx.pure.address(validator),
		],
	});
	return tx;
}

export function createUnstakeTransaction(stakedSuiId: string) {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: '0x3::sui_system::request_withdraw_stake',
		arguments: [tx.object(BFC_SYSTEM_STATE_OBJECT_ID), tx.object(stakedSuiId)],
	});
	return tx;
}
