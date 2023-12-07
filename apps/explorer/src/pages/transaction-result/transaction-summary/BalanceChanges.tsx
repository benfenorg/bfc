// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
	type BalanceChangeSummary,
	CoinFormat,
	useFormatCoin,
	type BalanceChange,
	useResolveSuiNSName,
	getRecognizedUnRecognizedTokenChanges,
} from '@mysten/core';
import { Heading, Text } from '@mysten/ui';
import { useMemo } from 'react';

import { AddressLink } from '~/ui/InternalLink';
import { TransactionBlockCard, TransactionBlockCardSection } from '~/ui/TransactionBlockCard';

interface BalanceChangesProps {
	changes: BalanceChangeSummary;
}

function BalanceChangeEntry({ change }: { change: BalanceChange }) {
	const { amount, coinType, recipient } = change;

	const [formatted, symbol] = useFormatCoin(amount, coinType, CoinFormat.FULL);

	const isPositive = BigInt(amount) > 0n;

	if (!change) {
		return null;
	}

	return (
		<div className="flex flex-col gap-2 py-3 first:pt-0 only:pb-0 only:pt-0">
			<div className="flex flex-col gap-2">
				<div className="flex h-6 flex-wrap items-center justify-between px-[7px]">
					<Text variant="pBody/medium" color="steel-dark">
						Amount
					</Text>
					<div className="flex">
						<Text variant="pBody/medium" color={isPositive ? 'bfc-green' : 'bfc-red'}>
							{isPositive ? '+' : ''}
							{formatted} {symbol}
						</Text>
					</div>
				</div>

				{recipient && (
					<div className="flex flex-wrap items-center justify-between">
						<Text variant="pBody/medium" color="steel-dark">
							Recipient
						</Text>
						<AddressLink address={recipient} />
					</div>
				)}
			</div>
		</div>
	);
}

function BalanceChangeCard({ changes, owner }: { changes: BalanceChange[]; owner: string }) {
	const { data: suinsDomainName } = useResolveSuiNSName(owner);
	const { recognizedTokenChanges, unRecognizedTokenChanges } = useMemo(
		() => getRecognizedUnRecognizedTokenChanges(changes),
		[changes],
	);

	return (
		<TransactionBlockCard
			title={
				<div className="flex w-full flex-wrap items-center justify-between gap-y-2">
					<Heading variant="heading6/semibold" color="steel-darker">
						Balance Changes
					</Heading>
				</div>
			}
			shadow
			size="sm"
			footer={
				owner ? (
					<div className="flex flex-wrap justify-between">
						<Text variant="pBody/medium" color="steel-dark">
							Owner
						</Text>
						<Text variant="pBody/medium" color="hero-dark">
							<AddressLink label={suinsDomainName || undefined} address={owner} />
						</Text>
					</div>
				) : null
			}
		>
			<div className="flex flex-col gap-2">
				{recognizedTokenChanges.map((change, index) => (
					<TransactionBlockCardSection key={index + change.coinType}>
						<BalanceChangeEntry change={change} />
					</TransactionBlockCardSection>
				))}
				{unRecognizedTokenChanges.length > 0 && (
					<div className="flex flex-col gap-2 border-t border-gray-45 pt-2">
						{unRecognizedTokenChanges.map((change, index) => (
							<TransactionBlockCardSection key={index + change.coinType}>
								<BalanceChangeEntry change={change} />
							</TransactionBlockCardSection>
						))}
					</div>
				)}
			</div>
		</TransactionBlockCard>
	);
}

export function BalanceChanges({ changes }: BalanceChangesProps) {
	if (!changes) return null;

	return (
		<>
			{Object.entries(changes).map(([owner, changes]) => (
				<BalanceChangeCard key={owner} changes={changes} owner={owner} />
			))}
		</>
	);
}
