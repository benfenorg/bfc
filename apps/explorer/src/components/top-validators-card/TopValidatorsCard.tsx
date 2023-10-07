// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useGetValidatorsEvents, useGetValidatorsApy } from '@mysten/core';
import { useLatestSuiSystemState } from '@mysten/dapp-kit';
import { ArrowRight12 } from '@mysten/icons';
import { type SuiValidatorSummary } from '@mysten/sui.js/client';
import { Text } from '@mysten/ui';
import { useMemo } from 'react';

import { validatorsTableData } from '../../pages/validators/utils';
import { Link } from '~/ui/Link';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';

const NUMBER_OF_VALIDATORS = 10;

export function processValidators(set: SuiValidatorSummary[]) {
	return set.map((av) => ({
		name: av.name,
		address: av.suiAddress,
		stake: av.stakingPoolSuiBalance,
		logo: av.imageUrl,
	}));
}

type TopValidatorsCardProps = {
	limit?: number;
	showIcon?: boolean;
};

export function TopValidatorsCard({ limit }: TopValidatorsCardProps) {
	const { data, isLoading, isSuccess, isError } = useLatestSuiSystemState();

	const numberOfValidators = data?.activeValidators.length || 0;

	const { data: validatorsApy } = useGetValidatorsApy();

	const {
		data: validatorEvents,
		isLoading: validatorsEventsLoading,
		isError: validatorEventError,
	} = useGetValidatorsEvents({
		limit: numberOfValidators,
		order: 'descending',
	});

	const tableData = useMemo(() => {
		if (!data || !validatorEvents) return null;
		let activeValidators = data?.activeValidators?.length ? [...data.activeValidators] : [];
		activeValidators = limit ? activeValidators.splice(0, limit) : activeValidators;
		return validatorsTableData(
			activeValidators,
			data.atRiskValidators,
			validatorEvents,
			validatorsApy || null,
		);
	}, [data, validatorEvents, validatorsApy, limit]);

	if (isError || validatorEventError) {
		return (
			<div className="px-3.5 pt-2 font-sans font-semibold text-issue-dark">
				Failed to load Validator
			</div>
		);
	}

	return (
		<div className="obc-table-container">
			{(isLoading || validatorsEventsLoading) && (
				<PlaceholderTable
					rowCount={limit || NUMBER_OF_VALIDATORS}
					rowHeight="13px"
					colHeadings={['Name', 'Address', 'Stake']}
					colWidths={['220px', '220px', '220px']}
				/>
			)}

			{isSuccess && tableData?.data && (
				<div>
					<TableCard data={tableData.data} columns={tableData.columns} />
					<div className="mt-3 flex justify-between bg-obc-card p-3.5">
						<Link to="/validators">
							<div className="flex items-center gap-2">
								View all
								<ArrowRight12 fill="currentColor" className="h-3 w-3 -rotate-45" />
							</div>
						</Link>
						<Text variant="body/normal" color="steel-darker">
							{data ? data.activeValidators.length : '-'}
							{` Total`}
						</Text>
					</div>
				</div>
			)}
		</div>
	);
}
