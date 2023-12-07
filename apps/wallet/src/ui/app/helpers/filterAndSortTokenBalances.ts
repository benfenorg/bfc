// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Coin } from '@benfen/bfc.js';
import { type CoinBalance } from '@benfen/bfc.js/client';

// Sort tokens by symbol and total balance
// Move this to the API backend
// Filter out tokens with zero balance
export function filterAndSortTokenBalances(tokens: CoinBalance[]) {
	return tokens
		.filter((token) => Number(token.totalBalance) > 0)
		.sort((a, b) =>
			(Coin.getCoinSymbol(a.coinType) + Number(a.totalBalance)).localeCompare(
				Coin.getCoinSymbol(b.coinType) + Number(b.totalBalance),
			),
		);
}
