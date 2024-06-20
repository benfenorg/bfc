// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CoinItem } from '_components/active-coins-card/CoinItem';
import { type CoinBalance } from '@benfen/bfc.js/client';
import { type ReactNode } from 'react';
import { Link } from 'react-router-dom';

type Props = {
	coinBalance: CoinBalance;
	centerAction?: ReactNode;
};

export function TokenLink({ coinBalance, centerAction }: Props) {
	return (
		<Link
			to={`/send?type=${encodeURIComponent(coinBalance.coinType)}`}
			key={coinBalance.coinType}
			className="no-underline w-full group/coin"
		>
			<CoinItem
				coinType={coinBalance.coinType}
				balance={BigInt(coinBalance.totalBalance)}
				centerAction={centerAction}
			/>
		</Link>
	);
}
