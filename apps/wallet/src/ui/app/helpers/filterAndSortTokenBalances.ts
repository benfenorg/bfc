// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { Coin } from '@benfen/bfc.js';
import { type CoinBalance } from '@benfen/bfc.js/client';
=======
import { type CoinBalance } from '@mysten/sui.js/client';
>>>>>>> mainnet-v1.24.1

// Sort tokens by symbol and total balance
// Move this to the API backend
// Filter out tokens with zero balance
export function filterAndSortTokenBalances(tokens: CoinBalance[]) {
	return tokens
		.filter((token) => Number(token.totalBalance) > 0)
		.sort((a, b) =>
			(getCoinSymbol(a.coinType) + Number(a.totalBalance)).localeCompare(
				getCoinSymbol(b.coinType) + Number(b.totalBalance),
			),
		);
}

export function getCoinSymbol(coinTypeArg: string) {
	return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
}
