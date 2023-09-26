// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useGetSystemState, useGetValidatorsEvents, useGetValidatorsApy } from '@mysten/core';
import { ArrowRight12 } from '@mysten/icons';
import { type SuiValidatorSummary } from '@mysten/sui.js';
import { Text } from '@mysten/ui';
import { useMemo } from 'react';

import { validatorsTableData } from '../../pages/validators/utils';
import { Link } from '~/ui/Link';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';
=======
import { useLatestSuiSystemState } from '@mysten/dapp-kit';
import { ArrowRight12 } from '@mysten/icons';
import { type SuiValidatorSummary } from '@mysten/sui.js/client';
import { Text } from '@mysten/ui';
import { useMemo } from 'react';

import { StakeColumn } from './StakeColumn';
import { HighlightedTableCol } from '~/components/Table/HighlightedTableCol';
import { Banner } from '~/ui/Banner';
import { ImageIcon } from '~/ui/ImageIcon';
import { AddressLink, ValidatorLink } from '~/ui/InternalLink';
import { Link } from '~/ui/Link';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';
import { ampli } from '~/utils/analytics/ampli';
>>>>>>> heads/mainnet-v1.9.1

const NUMBER_OF_VALIDATORS = 10;

export function processValidators(set: SuiValidatorSummary[]) {
	return set.map((av) => ({
		name: av.name,
		address: av.suiAddress,
		stake: av.stakingPoolSuiBalance,
		logo: av.imageUrl,
	}));
}

<<<<<<< HEAD
=======
const validatorsTable = (
	validatorsData: SuiValidatorSummary[],
	limit?: number,
	showIcon?: boolean,
) => {
	const validators = processValidators(validatorsData).sort((a, b) =>
		Math.random() > 0.5 ? -1 : 1,
	);

	const validatorsItems = limit ? validators.splice(0, limit) : validators;

	return {
		data: validatorsItems.map(({ name, stake, address, logo }) => ({
			name: (
				<HighlightedTableCol first>
					<div className="flex items-center gap-2.5">
						{showIcon && <ImageIcon src={logo} size="sm" fallback={name} label={name} circle />}

						<ValidatorLink
							address={address}
							label={name}
							onClick={() =>
								ampli.clickedValidatorRow({
									sourceFlow: 'Top validators - validator name',
									validatorAddress: address,
									validatorName: name,
								})
							}
						/>
					</div>
				</HighlightedTableCol>
			),
			stake: <StakeColumn stake={stake} />,
			delegation: (
				<Text variant="bodySmall/medium" color="steel-darker">
					{stake.toString()}
				</Text>
			),
			address: (
				<HighlightedTableCol>
					<AddressLink
						address={address}
						noTruncate={!limit}
						onClick={() =>
							ampli.clickedValidatorRow({
								sourceFlow: 'Top validators - validator address',
								validatorAddress: address,
								validatorName: name,
							})
						}
					/>
				</HighlightedTableCol>
			),
		})),
		columns: [
			{
				header: 'Name',
				accessorKey: 'name',
			},
			{
				header: 'Address',
				accessorKey: 'address',
			},
			{
				header: 'Stake',
				accessorKey: 'stake',
			},
		],
	};
};

>>>>>>> heads/mainnet-v1.9.1
type TopValidatorsCardProps = {
	limit?: number;
	showIcon?: boolean;
};

<<<<<<< HEAD
export function TopValidatorsCard({ limit }: TopValidatorsCardProps) {
	const { data, isLoading, isSuccess, isError } = useGetSystemState();

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
=======
export function TopValidatorsCard({ limit, showIcon }: TopValidatorsCardProps) {
	const { data, isLoading, isSuccess, isError } = useLatestSuiSystemState();

	const tableData = useMemo(
		() => (data ? validatorsTable(data.activeValidators, limit, showIcon) : null),
		[data, limit, showIcon],
	);

	if (isError || (!isLoading && !tableData?.data.length)) {
		return (
			<Banner variant="error" fullWidth>
				Validator data could not be loaded
			</Banner>
		);
	}

	return (
		<>
			{isLoading && (
				<PlaceholderTable
					rowCount={limit || NUMBER_OF_VALIDATORS}
					rowHeight="13px"
					colHeadings={['Name', 'Address', 'Stake']}
					colWidths={['220px', '220px', '220px']}
				/>
			)}

			{isSuccess && tableData && (
				<>
					<TableCard data={tableData.data} columns={tableData.columns} />
					<div className="mt-3 flex justify-between">
>>>>>>> heads/mainnet-v1.9.1
						<Link to="/validators">
							<div className="flex items-center gap-2">
								View all
								<ArrowRight12 fill="currentColor" className="h-3 w-3 -rotate-45" />
							</div>
						</Link>
<<<<<<< HEAD
						<Text variant="body/normal" color="steel-darker">
=======
						<Text variant="body/medium" color="steel-dark">
>>>>>>> heads/mainnet-v1.9.1
							{data ? data.activeValidators.length : '-'}
							{` Total`}
						</Text>
					</div>
<<<<<<< HEAD
				</div>
			)}
		</div>
=======
				</>
			)}
		</>
>>>>>>> heads/mainnet-v1.9.1
	);
}
