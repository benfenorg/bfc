// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useGetValidatorsApy, useGetValidatorsEvents } from '@mysten/core';
import { useLatestSuiSystemState } from '@mysten/dapp-kit';
import { ArrowLeft12 } from '@mysten/icons';
import { useMemo } from 'react';
import { useNavigate } from 'react-router-dom';

import { validatorsTableData } from './utils';
import { PageLayout } from '~/components/Layout/PageLayout';
import { ErrorBoundary } from '~/components/error-boundary/ErrorBoundary';
import { Banner } from '~/ui/Banner';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';

function ValidatorPageResult() {
	const { data, isLoading, isSuccess, isError } = useLatestSuiSystemState();

	const numberOfValidators = data?.activeValidators.length || 0;

	const {
		data: validatorEvents,
		isLoading: validatorsEventsLoading,
		isError: validatorEventError,
	} = useGetValidatorsEvents({
		limit: numberOfValidators,
		order: 'descending',
	});

	const { data: validatorsApy } = useGetValidatorsApy();

	const validatorsTable = useMemo(() => {
		if (!data || !validatorEvents) return null;
		return validatorsTableData(
			data.activeValidators,
			data.atRiskValidators,
			validatorEvents,
			validatorsApy || null,
		);
	}, [data, validatorEvents, validatorsApy]);

	const navigate = useNavigate();

	const pageBack = () => {
		navigate(-1);
	};

	return (
		<PageLayout
			content={
				isError || validatorEventError ? (
					<Banner variant="error" fullWidth>
						Validator data could not be loaded
					</Banner>
				) : (
					<div className="">
						<ErrorBoundary>
							<div className="relative mb-6 flex cursor-pointer items-center" onClick={pageBack}>
								<ArrowLeft12 width={20} height={20} />
								<span className="ml-2 text-base font-semibold capitalize">Validators</span>
							</div>
							{(isLoading || validatorsEventsLoading) && (
								<PlaceholderTable
									rowCount={20}
									rowHeight="13px"
									colHeadings={['Name', 'Address', 'Stake']}
									colWidths={['220px', '220px', '220px']}
								/>
							)}

							{isSuccess && validatorsTable?.data && (
								<div className="bfc-table-container">
									<TableCard data={validatorsTable.data} columns={validatorsTable.columns} />
								</div>
							)}
						</ErrorBoundary>
					</div>
				)
			}
		/>
	);
}

export { ValidatorPageResult };
