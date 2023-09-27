// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';
import { memo } from 'react';

export type CoinProps = {
	type: string;
	balance: bigint;
};

function CoinBalance({ type, balance }: CoinProps) {
	const [formatted, symbol] = useFormatCoin(balance, type);

	return (
		<div className="flex items-end gap-1.25" role="button">
			<span className="text-[32px] text-obc font-bold">{formatted}</span>
			<span className="text-[20px]/[32px] text-obc-text3 font-bold">{symbol}</span>
		</div>
	);
}

export default memo(CoinBalance);
