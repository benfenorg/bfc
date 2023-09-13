// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';
import { SUI_TYPE_ARG } from '@mysten/sui.js';

import { Heading } from '_app/shared/heading';
import { Text } from '_app/shared/text';

//TODO unify StakeAmount and CoinBalance
interface StakeAmountProps {
	balance: bigint | number | string;
	variant: 'heading5' | 'body';
	isEarnedRewards?: boolean;
}

export function StakeAmount({ balance, variant, isEarnedRewards }: StakeAmountProps) {
	const [formatted, symbol] = useFormatCoin(balance, SUI_TYPE_ARG);
	// Handle case of 0 balance
	let colorAmount: 'obc-text1' | 'obc-text3' = 'obc-text1';
	let colorSymbol: 'obc-text1' | 'obc-text3' = 'obc-text1';
	if (isEarnedRewards) {
		colorSymbol = 'obc-text3';
	}
	if (formatted === '0') {
		colorAmount = 'obc-text3';
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
