// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { formatAmount, formatDate, useRpcClient } from '@mysten/core';
=======
import { formatAmount, formatDate } from '@mysten/core';
import { useSuiClient, useTotalTransactionBlocks } from '@mysten/dapp-kit';
>>>>>>> heads/mainnet-v1.9.1
import { Heading, Text, LoadingIndicator } from '@mysten/ui';
import { useQuery } from '@tanstack/react-query';
import { ParentSize } from '@visx/responsive';
import clsx from 'clsx';

import { AreaGraph } from './AreaGraph';
<<<<<<< HEAD
=======
import { FormattedStatsAmount } from './HomeMetrics/FormattedStatsAmount';
>>>>>>> heads/mainnet-v1.9.1
import { ErrorBoundary } from './error-boundary/ErrorBoundary';
import { Card } from '~/ui/Card';

function TooltipContent({
	data: { epochTotalTransactions, epochStartTimestamp, epoch },
}: {
	data: {
		epochTotalTransactions: number;
		epochStartTimestamp: number;
		epoch: number;
	};
}) {
	const dateFormatted = formatDate(new Date(epochStartTimestamp), ['day', 'month']);
	const totalFormatted = formatAmount(epochTotalTransactions);
	return (
		<div className="flex flex-col gap-0.5">
			<Text variant="subtitleSmallExtra/medium" color="steel-darker">
				{dateFormatted}, Epoch {epoch}
			</Text>
			<Heading variant="heading6/semibold" color="steel-darker">
				{totalFormatted}
			</Heading>
			<Text variant="subtitleSmallExtra/medium" color="steel-darker" uppercase>
				Transaction Blocks
			</Text>
		</div>
	);
}

function useEpochTransactions() {
<<<<<<< HEAD
	const rpc = useRpcClient();
=======
	const client = useSuiClient();
>>>>>>> heads/mainnet-v1.9.1
	return useQuery({
		queryKey: ['get', 'last', '30', 'epoch', 'transactions'],
		queryFn: async () =>
			[
				...(
<<<<<<< HEAD
					await rpc.getEpochs({
=======
					await client.getEpochs({
>>>>>>> heads/mainnet-v1.9.1
						descendingOrder: true,
						limit: 31,
					})
				).data,
<<<<<<< HEAD
				// ...aaa
=======
>>>>>>> heads/mainnet-v1.9.1
			]
				.reverse()
				.slice(0, -1),
		select: (data) =>
			data.map(({ epoch, epochTotalTransactions, epochStartTimestamp }) => ({
				epoch: Number(epoch),
				epochTotalTransactions: Number(epochTotalTransactions),
				epochStartTimestamp: Number(epochStartTimestamp),
			})),
	});
}

export function TransactionsCardGraph() {
<<<<<<< HEAD
	const { data: epochMetrics, isLoading } = useEpochTransactions();

	return (
		<Card bg="white" spacing={!epochMetrics?.length ? 'lg' : 'lgGraph'} height="full">
			<div className="flex h-full flex-col gap-4 overflow-hidden">
				<div className="heading6 font-bold text-[#171719]">Transaction Blocks</div>
=======
	const { data: totalTransactions } = useTotalTransactionBlocks(
		{},
		{
			cacheTime: 24 * 60 * 60 * 1000,
			staleTime: Infinity,
			retry: 5,
		},
	);
	const { data: epochMetrics, isLoading } = useEpochTransactions();
	const lastEpochTotalTransactions =
		epochMetrics?.[epochMetrics.length - 1]?.epochTotalTransactions;

	return (
		<Card bg="white/80" spacing={!epochMetrics?.length ? 'lg' : 'lgGraph'} height="full">
			<div className="flex h-full flex-col gap-4 overflow-hidden">
				<Heading variant="heading4/semibold" color="steel-darker">
					Transaction Blocks
				</Heading>
				<div className="flex flex-wrap gap-6">
					<FormattedStatsAmount
						orientation="vertical"
						label="Total"
						tooltip="Total transaction blocks"
						amount={totalTransactions}
						size="md"
					/>
					<FormattedStatsAmount
						orientation="vertical"
						label="Last Epoch"
						amount={lastEpochTotalTransactions}
						size="md"
					/>
				</div>
>>>>>>> heads/mainnet-v1.9.1
				<div
					className={clsx(
						'flex min-h-[180px] flex-1 flex-col items-center justify-center rounded-xl transition-colors',
						!epochMetrics?.length && 'bg-gray-40',
					)}
				>
					{isLoading ? (
						<div className="flex flex-col items-center gap-1">
							<LoadingIndicator />
							<Text color="steel" variant="body/medium">
								loading data
							</Text>
						</div>
					) : epochMetrics?.length ? (
						<div className="relative flex-1 self-stretch">
							<ErrorBoundary>
								<ParentSize className="absolute">
									{({ height, width }) => (
										<AreaGraph
											data={epochMetrics}
											height={height}
											width={width}
											getX={({ epoch }) => Number(epoch)}
											getY={({ epochTotalTransactions }) => Number(epochTotalTransactions)}
<<<<<<< HEAD
											color="black"
=======
											color="yellow"
>>>>>>> heads/mainnet-v1.9.1
											formatY={formatAmount}
											tooltipContent={TooltipContent}
										/>
									)}
								</ParentSize>
							</ErrorBoundary>
						</div>
					) : (
						<Text color="steel" variant="body/medium">
							No historical data available
						</Text>
					)}
				</div>
			</div>
		</Card>
	);
}
