// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type CoinStruct } from '@benfen/bfc.js/client';
import { TransactionBlock } from '@benfen/bfc.js/transactions';
import { SUI_SYSTEM_STATE_OBJECT_ID, SUI_TYPE_ARG } from '@benfen/bfc.js/utils';

export function createStakeTransaction(
	amount: bigint,
	validator: string,
	coinType: string = SUI_TYPE_ARG,
	coins: CoinStruct[] = [],
) {
	const tx = new TransactionBlock();
	let stakeCoin: ReturnType<typeof tx.splitCoins>;
	if (coinType === SUI_TYPE_ARG) {
		stakeCoin = tx.splitCoins(tx.gas, [tx.pure(amount)]);
	} else {
		const coin = coins.find((c) => BigInt(c.balance) > amount);
		if (!coin) {
			throw new Error('insuficient balance');
		}
		stakeCoin = tx.splitCoins(tx.object(coin.coinObjectId), [tx.pure(amount)]);
	}
	const target = {
		[SUI_TYPE_ARG]: '0x3::sui_system::request_add_stake',
		'0xc8::busd::BUSD': '0x3::sui_system::request_add_stable_stake',
	}[coinType]!;

	tx.moveCall({
		target: target as any,
		arguments: [
			tx.sharedObjectRef({
				objectId: SUI_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			}),
			stakeCoin,
			tx.pure(validator, 'address'),
		],
	});
	return tx;
}

export function createUnstakeTransaction(stakedSuiId: string) {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: '0x3::sui_system::request_withdraw_stake',
		arguments: [tx.object(SUI_SYSTEM_STATE_OBJECT_ID), tx.object(stakedSuiId)],
	});
	return tx;
}
