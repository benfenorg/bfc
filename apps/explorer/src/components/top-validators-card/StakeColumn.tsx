// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin, CoinFormat, formatBalance } from '@mysten/core';
import { SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { Text } from '@mysten/ui';

type StakeColumnProps = {
	stake: bigint | number | string;
	hideCoinSymbol?: boolean;
	inMIST?: boolean;
};

export function StakeColumn({ stake, hideCoinSymbol, inMIST = false }: StakeColumnProps) {
	const coinFormat = hideCoinSymbol ? CoinFormat.FULL : CoinFormat.ROUNDED;
	const [amount, symbol] = useFormatCoin(stake, SUI_TYPE_ARG, coinFormat);
	return (
		<div className="flex items-end gap-0.5">
			<Text variant="pBody/normal" color="steel-darker">
				{inMIST ? formatBalance(stake, 0, coinFormat) : amount}
			</Text>
			{!hideCoinSymbol && (
				<Text variant="pBody/normal" color="steel-dark">
					{inMIST ? 'MIST' : symbol}
				</Text>
			)}
		</div>
	);
}
