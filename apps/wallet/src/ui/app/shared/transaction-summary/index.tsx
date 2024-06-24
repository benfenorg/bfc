// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { type TransactionSummary as TransactionSummaryType } from '@mysten/core';

import LoadingIndicator from '../../components/loading/LoadingIndicator';
import { Heading } from '../heading';
import { BalanceChanges } from './cards/BalanceChanges';
import { GasSummary } from './cards/GasSummary';
import { ObjectChanges } from './cards/ObjectChanges';

export function TransactionSummary({
	summary,
	isLoading,
	isError,
	isDryRun = false,
	/* todo: remove this, we're using it until we update tx approval page */
	showGasSummary = false,
}: {
	summary: TransactionSummaryType;
	isLoading?: boolean;
	isDryRun?: boolean;
	isError?: boolean;
	showGasSummary?: boolean;
}) {
	if (isError) return null;
	return (
		<section className="min-h-full pb-5">
			{isLoading ? (
				<div className="flex items-center justify-center p-10">
					<LoadingIndicator />
				</div>
			) : (
				<div className="flex flex-col gap-5">
					{isDryRun && (
						<div className="flex justify-center">
							<Heading variant="heading6" color="bfc-text1">
								Do you approve these actions?
							</Heading>
						</div>
					)}
					<BalanceChanges changes={summary?.balanceChanges} />
					<ObjectChanges changes={summary?.objectSummary} />
					{showGasSummary && <GasSummary gasSummary={summary?.gas} />}
				</div>
			)}
		</section>
	);
}
