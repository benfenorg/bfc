// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { type CoinBalance } from '@benfen/bfc.js';
import { type ReactNode } from 'react';
import { Link } from 'react-router-dom';

import { CoinItem } from '_components/active-coins-card/CoinItem';
=======
import { CoinItem } from '_components/active-coins-card/CoinItem';
import { ampli } from '_src/shared/analytics/ampli';
import { type CoinBalance } from '@mysten/sui.js/client';
import { MIST_PER_SUI } from '@mysten/sui.js/utils';
import { type ReactNode } from 'react';
import { Link } from 'react-router-dom';
>>>>>>> mainnet-v1.24.1

type Props = {
	coinBalance: CoinBalance;
	centerAction?: ReactNode;
	subtitle?: string;
};

export function TokenLink({ coinBalance, centerAction, subtitle }: Props) {
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
				subtitle={subtitle}
			/>
		</Link>
	);
}
