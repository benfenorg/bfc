// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { getObjectType } from '@benfen/bfc.js';

import type { SuiObjectData, SuiMoveObject } from '@benfen/bfc.js';
=======
import type { SuiMoveObject, SuiObjectData } from '@mysten/sui.js/client';
>>>>>>> mainnet-v1.24.1

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

export const GAS_TYPE_ARG = '0x2::bfc::BFC';
export const GAS_SYMBOL = 'BFC';

// TODO use sdk
export class Coin {
	public static isCoin(obj: SuiObjectData) {
		return getObjectType(obj)?.startsWith(COIN_TYPE) ?? false;
	}

	public static getCoinTypeArg(obj: SuiMoveObject) {
		const res = obj.type.match(COIN_TYPE_ARG_REGEX);
		return res ? res[1] : null;
	}

	public static isSUI(obj: SuiMoveObject) {
		const arg = Coin.getCoinTypeArg(obj);
		return arg ? Coin.getCoinSymbol(arg) === 'BFC' : false;
	}

	public static getCoinSymbol(coinTypeArg: string) {
		return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
	}

	public static getBalance(obj: SuiMoveObject): bigint {
		return BigInt(obj.fields.balance);
	}

	public static getID(obj: SuiMoveObject): string {
		return obj.fields.id.id;
	}

	public static getCoinTypeFromArg(coinTypeArg: string) {
		return `${COIN_TYPE}<${coinTypeArg}>`;
	}
}
