// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';
import { useLatestSuiSystemState } from '@mysten/dapp-kit';
import { SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { LoadingIndicator } from '@mysten/ui';
import { useQuery } from '@tanstack/react-query';
import clsx from 'clsx';
import { useMemo } from 'react';
import { useParams } from 'react-router-dom';

import { EpochProgress } from './stats/EpochProgress';
import { EpochStats } from './stats/EpochStats';
import { ValidatorStatus } from './stats/ValidatorStatus';
import { validatorsTableData } from '../validators/utils';
import { PageLayout } from '~/components/Layout/PageLayout';
import { CheckpointsTable } from '~/components/checkpoints/CheckpointsTable';
import { useEnhancedRpcClient } from '~/hooks/useEnhancedRpc';
import { Banner } from '~/ui/Banner';
import { PageBackHeader } from '~/ui/PageBackHeader';
import { Stats, type StatsProps } from '~/ui/Stats';
import { TableCard } from '~/ui/TableCard';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '~/ui/Tabs';
import { getEpochStorageFundFlow } from '~/utils/getStorageFundFlow';

function SuiStats({
	amount,
	...props
}: Omit<StatsProps, 'children'> & {
	amount: bigint | number | string | undefined | null;
}) {
	const [formattedAmount, symbol] = useFormatCoin(amount, SUI_TYPE_ARG);

	return (
		<Stats postfix={formattedAmount && symbol} {...props} darker>
			{formattedAmount || '--'}
		</Stats>
	);
}

export default function EpochDetail() {
	const { id } = useParams();
	const enhancedRpc = useEnhancedRpcClient();
	const { data: systemState } = useLatestSuiSystemState();
	const { data, isLoading, isError } = useQuery({
		queryKey: ['epoch', id],
		queryFn: async () =>
			enhancedRpc.getEpochs({
				// todo: endpoint returns no data for epoch 0
				cursor: id === '0' ? undefined : (Number(id!) - 1).toString(),
				limit: 1,
			}),
	});

	const [epochData] = data?.data ?? [];
	const isCurrentEpoch = useMemo(
		() => systemState?.epoch === epochData?.epoch,
		[systemState, epochData],
	);

	const validatorsTable = useMemo(() => {
		if (!epochData?.validators) return null;
		// todo: enrich this historical validator data when we have
		// at-risk / pending validators for historical epochs
		return validatorsTableData(
			[...epochData.validators].sort(() => 0.5 - Math.random()),
			[],
			[],
			null,
		);
	}, [epochData]);

	if (isLoading) return <PageLayout content={<LoadingIndicator />} />;

	if (isError || !epochData)
		return (
			<PageLayout
				content={
					<Banner variant="error" fullWidth>
						{`There was an issue retrieving data for epoch ${id}.`}
					</Banner>
				}
			/>
		);

	const { fundInflow, fundOutflow, netInflow } = getEpochStorageFundFlow(epochData.endOfEpochInfo);

	// cursor should be the sequence number of the last checkpoint + 1  if we want to query with desc. order
	const initialCursorPlusOne = epochData.endOfEpochInfo?.lastCheckpointId
		? (Number(epochData.endOfEpochInfo?.lastCheckpointId) + 1).toString()
		: undefined;

	return (
		<PageLayout
			content={
				<div className="flex flex-col">
					<div>
						<PageBackHeader title="Epochs Details" />
					</div>
					<div
						className={clsx(
							'grid gap-4 max-sm:grid-cols-1 sm:gap-2 md:gap-5',
							isCurrentEpoch ? 'grid-cols-4' : 'grid-cols-3',
						)}
					>
						<EpochProgress
							epoch={epochData.epoch}
							inProgress={isCurrentEpoch}
							start={Number(epochData.epochStartTimestamp)}
							end={Number(epochData.endOfEpochInfo?.epochEndTimestamp ?? 0)}
						/>

						<EpochStats label="Rewards">
							<SuiStats
								label="Total Stake"
								tooltip=""
								amount={epochData.endOfEpochInfo?.totalStake}
							/>
							<SuiStats
								label="Stake Subsidies"
								amount={epochData.endOfEpochInfo?.stakeSubsidyAmount}
							/>
							<SuiStats
								label="Stake Rewards"
								amount={epochData.endOfEpochInfo?.totalStakeRewardsDistributed}
							/>
							<SuiStats label="Gas Fees" amount={epochData.endOfEpochInfo?.totalGasFees} />
						</EpochStats>

						<EpochStats label="Storage Fund Balance">
							<SuiStats label="Fund Size" amount={epochData.endOfEpochInfo?.storageFundBalance} />
							<SuiStats label="Net Inflow" amount={netInflow} />
							<SuiStats label="Fund Inflow" amount={fundInflow} />
							<SuiStats label="Fund Outflow" amount={fundOutflow} />
						</EpochStats>

						{isCurrentEpoch ? <ValidatorStatus /> : null}
					</div>

					<Tabs size="md" defaultValue="checkpoints" className="mt-5 ">
						<TabsList disableBottomBorder>
							<TabsTrigger value="checkpoints">Checkpoints</TabsTrigger>
							<TabsTrigger value="validators">Participating Validators</TabsTrigger>
						</TabsList>
						<TabsContent value="checkpoints">
							<CheckpointsTable
								initialCursor={initialCursorPlusOne}
								maxCursor={epochData.firstCheckpointId}
								initialLimit={20}
							/>
						</TabsContent>
						<TabsContent value="validators">
							{validatorsTable ? (
								<div className="bfc-table-container">
									<TableCard data={validatorsTable.data} columns={validatorsTable.columns} />
								</div>
							) : null}
						</TabsContent>
					</Tabs>
				</div>
			}
		/>
	);
}
