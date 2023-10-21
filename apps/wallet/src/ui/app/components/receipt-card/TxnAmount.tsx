// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';

import { Heading } from '_src/ui/app/shared/heading';
import { Text } from '_src/ui/app/shared/text';

type TxnAmountProps = {
	amount: string | number;
	coinType: string;
	label: string;
	approximation?: boolean;
};

// dont show amount if it is 0
// This happens when a user sends a transaction to self;
export function TxnAmount({ amount, coinType, label, approximation }: TxnAmountProps) {
	const [formatAmount, symbol] = useFormatCoin(Math.abs(Number(amount)), coinType);
	return Number(amount) !== 0 ? (
		<div className="h-10 flex justify-between w-full items-center">
			<Text variant="body" weight="normal" color="bfc-text1">
				{label}
			</Text>
			<div className="flex gap-1 items-center">
				<Heading variant="heading3" weight="bold" color="bfc-text1">
					{approximation ? '~' : ''}
					{formatAmount}
				</Heading>
				<Text variant="body" weight="normal" color="bfc-text1">
					{symbol}
				</Text>
			</div>
		</div>
	) : null;
}
