// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import BigNumber from 'bignumber.js';

export function parseAmount(amount: string, coinDecimals: number) {
	try {
		return BigInt(new BigNumber(amount).shiftedBy(coinDecimals).integerValue().toString());
	} catch (e) {
		return BigInt(0);
	}
}
