// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { getRefGasPrice } from '@mysten/core';
import { useLatestSuiSystemState } from '@mysten/dapp-kit';
import { Heading, Text } from '@mysten/ui';
import { useMemo } from 'react';

import { Card } from '~/ui/Card';
import { Divider } from '~/ui/Divider';
import { RingChart, RingChartLegend } from '~/ui/RingChart';

export function ValidatorStatus() {
	const { data } = useLatestSuiSystemState();

	const nextRefGasPrice = useMemo(
		() => getRefGasPrice(data?.activeValidators),
		[data?.activeValidators],
	);

	if (!data) return null;

	const nextEpoch = Number(data.epoch || 0) + 1;

	const chartData = [
		{
			value: data.activeValidators.length,
			label: 'Active',
			gradient: {
				deg: 315,
				values: [
					{ percent: 0, color: '#4C75A6' },
					{ percent: 100, color: '#589AEA' },
				],
			},
		},
		{
			value: Number(data.pendingActiveValidatorsSize ?? 0),
			label: 'New',
			color: '#F2BD24',
		},
		{
			value: data.atRiskValidators.length,
			label: 'At Risk',
			color: '#FF794B',
		},
	];

	return (
		<Card spacing="lg" bg="white" border="obcBorder" rounded="2xl">
			<div className="flex items-center gap-5">
				<div className="min-h-[96px] min-w-[96px]">
					<RingChart data={chartData} />
				</div>

				<div className="self-start">
					<RingChartLegend data={chartData} title={`Validators in Epoch ${nextEpoch}`} />
				</div>
			</div>
			<div className="mt-5">
				<Divider />
			</div>
			<div className="mt-5">
				<div>
					<Text variant="pSubtitle/normal" color="steel-dark">
						Estimated Next Epoch Reference Gas Price
					</Text>
				</div>
				<div className="flex items-baseline gap-1">
					<Heading variant="heading4/semibold" color="steel-darker">
						{nextRefGasPrice.toString()}
					</Heading>
					<div>
						<Text variant="pBody/normal" color="steel">
							MIST
						</Text>
					</div>
				</div>
			</div>
		</Card>
	);
}
