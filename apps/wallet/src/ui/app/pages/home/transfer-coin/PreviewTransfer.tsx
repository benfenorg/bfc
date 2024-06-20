// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import { TxnAddress } from '_components/receipt-card/TxnAddress';
import { TxnAmount } from '_components/receipt-card/TxnAmount';
import { parseAmount } from '_helpers';
import { useActiveAddress } from '_src/ui/app/hooks/useActiveAddress';
import { GAS_SYMBOL } from '_src/ui/app/redux/slices/sui-objects/Coin';
import { Heading } from '_src/ui/app/shared/heading';
import { useCoinMetadata } from '@mysten/core';

export type PreviewTransferProps = {
	coinType: string;
	to: string;
	amount: string;
	approximation?: boolean;
	gasBudget?: string;
};

export function PreviewTransfer({
	coinType,
	to,
	amount,
	approximation,
	gasBudget,
}: PreviewTransferProps) {
	const accountAddress = useActiveAddress();
	const { data: metadata } = useCoinMetadata(coinType);
	const amountWithoutDecimals = parseAmount(amount, metadata?.decimals ?? 0);

	return (
		<div className="divide-y divide-solid divide-bfc-border divide-x-0 flex flex-col w-full [&>div]:pt-2.5 first:pt-0">
			<TxnAmount
				amount={amountWithoutDecimals.toString()}
				label="Sending"
				coinType={coinType}
				approximation={approximation}
			/>
			<TxnAddress address={accountAddress || ''} label="From" />
			<TxnAddress address={to} label="To" />
			<div className="h-10 flex w-full justify-between">
				<div className="flex items-center">
					<Text variant="body" color="bfc-text1" weight="normal">
						Estimated Gas Fees
					</Text>
				</div>
				<div className="flex items-center">
					<Heading variant="heading3" color="bfc-text1" weight="bold">
						{gasBudget}
					</Heading>
					&nbsp;
					<Text variant="body" color="bfc-text1" weight="normal">
						{GAS_SYMBOL}
					</Text>
				</div>
			</div>
		</div>
	);
}
