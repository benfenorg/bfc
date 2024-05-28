// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import Alert from '_components/alert';
import { CoinIcon } from '_src/ui/app/components/coin-icon';
import { Text } from '_src/ui/app/shared/text';
import {
	CoinFormat,
<<<<<<< HEAD
	useFormatCoin,
	type BalanceChange,
} from '@mysten/core';

import { Card } from '../Card';
import { OwnerFooter } from '../OwnerFooter';
import { CoinIcon } from '_src/ui/app/components/coin-icon';
import { Text } from '_src/ui/app/shared/text';
=======
	getRecognizedUnRecognizedTokenChanges,
	useCoinMetadata,
	useFormatCoin,
	type BalanceChange,
	type BalanceChangeSummary,
} from '@mysten/core';
import classNames from 'clsx';
import { useMemo } from 'react';

import { Card } from '../Card';
import { OwnerFooter } from '../OwnerFooter';
>>>>>>> mainnet-v1.24.1

interface BalanceChangesProps {
	changes?: BalanceChangeSummary;
}

function BalanceChangeEntry({ change }: { change: BalanceChange }) {
	const { amount, coinType } = change;
	const isPositive = BigInt(amount) > 0n;

	const [formatted, symbol] = useFormatCoin(amount, coinType, CoinFormat.FULL);

	return (
		<div className="h-15 px-2.5 flex justify-between items-center">
			<div className="flex items-center gap-1.25">
				<CoinIcon coinType={coinType} size="sm" />
				<Text variant="body" weight="medium" color="bfc-text1">
					{symbol}
				</Text>
			</div>
<<<<<<< HEAD
			<div className="flex">
				<Text variant="body" weight="medium" color={isPositive ? 'bfc-text1' : 'bfc-red'}>
					{isPositive ? '+' : ''}
					{formatted} {symbol}
				</Text>
=======
		</div>
	);
}

function BalanceChangeEntries({ changes }: { changes: BalanceChange[] }) {
	const { recognizedTokenChanges, unRecognizedTokenChanges } = useMemo(
		() => getRecognizedUnRecognizedTokenChanges(changes),
		[changes],
	);

	return (
		<div className="flex flex-col gap-2">
			<div className="flex flex-col gap-4 pb-3">
				{recognizedTokenChanges.map((change) => (
					<BalanceChangeEntry change={change} key={change.coinType + change.amount} />
				))}
				{unRecognizedTokenChanges.length > 0 && (
					<div
						className={classNames(
							'flex flex-col gap-2 pt-2',
							recognizedTokenChanges?.length && 'border-t border-gray-45',
						)}
					>
						{unRecognizedTokenChanges.map((change, index) => (
							<BalanceChangeEntry change={change} key={change.coinType + index} />
						))}
					</div>
				)}
>>>>>>> mainnet-v1.24.1
			</div>
		</div>
	);
}

export function BalanceChanges({ changes }: BalanceChangesProps) {
	if (!changes) return null;
	return (
		<>
			{Object.entries(changes).map(([owner, changes]) => (
				<Card heading="Balance Changes" key={owner} footer={<OwnerFooter owner={owner} />}>
					<div className="flex flex-col">
						{changes.map((change) => (
							<BalanceChangeEntry change={change} key={change.coinType + change.amount} />
						))}
					</div>
				</Card>
			))}
		</>
	);
}
