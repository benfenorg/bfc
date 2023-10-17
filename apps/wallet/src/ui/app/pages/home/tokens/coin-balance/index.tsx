// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';

export type CoinProps = {
	type: string;
	balance: bigint;
};

export default function CoinBalance({ type, balance }: CoinProps) {
	const [formatted, symbol] = useFormatCoin(balance, type);

	return (
		<div className="flex items-end gap-1.25" role="button">
			<span className="text-[32px] text-bfc font-bold">{formatted}</span>
			<span className="text-[20px]/[32px] text-bfc-text3 font-bold">{symbol}</span>
		</div>
	);
}
