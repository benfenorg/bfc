// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Heading } from '_app/shared/heading';
import { Text } from '_app/shared/text';
import { SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { useFormatCoin } from '@mysten/core';

//TODO unify StakeAmount and CoinBalance
interface StakeAmountProps {
	balance: bigint | number | string;
	variant: 'heading5' | 'body';
	isEarnedRewards?: boolean;
}

export function StakeAmount({ balance, variant, isEarnedRewards }: StakeAmountProps) {
	const [formatted, symbol] = useFormatCoin(balance, SUI_TYPE_ARG);
	// Handle case of 0 balance
	let colorAmount: 'bfc-text1' | 'bfc-text3' = 'bfc-text1';
	let colorSymbol: 'bfc-text1' | 'bfc-text3' = 'bfc-text1';
	if (isEarnedRewards) {
		colorSymbol = 'bfc-text3';
	}
	if (formatted === '0') {
		colorAmount = 'bfc-text3';
	}

	return (
		<div className="flex gap-0.5 align-baseline flex-nowrap items-baseline">
			{variant === 'heading5' ? (
				<Heading variant="heading5" as="div" weight="semibold" color={colorAmount}>
					{formatted}
				</Heading>
			) : (
				<Text variant={variant} weight="medium" color={colorAmount}>
					{formatted}
				</Text>
			)}

			<Text
				variant={variant === 'heading5' ? 'bodySmall' : 'body'}
				color={colorSymbol}
				weight="medium"
			>
				&nbsp;{symbol}
			</Text>
		</div>
	);
}
