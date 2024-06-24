// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { useExplorerLink } from '_src/ui/app/hooks/useExplorerLink';
import { useRecognizedPackages } from '_src/ui/app/hooks/useRecognizedPackages';
import { type SuiTransactionBlockResponse } from '@benfen/bfc.js/client';
import { useTransactionSummary } from '@mysten/core';
import { Handclap } from '@mysten/icons';

import { DateCard } from '../../shared/date-card';
import { TransactionSummary } from '../../shared/transaction-summary';
import { GasSummary } from '../../shared/transaction-summary/cards/GasSummary';
import { StakeTxnCard } from './StakeTxnCard';
import { UnStakeTxnCard } from './UnstakeTxnCard';

type ReceiptCardProps = {
	txn: SuiTransactionBlockResponse;
	activeAddress: string;
};

function TransactionStatus({
	success,
	timestamp,
}: {
	success: boolean;
	timestamp?: string | null;
}) {
	return (
		<div className="flex flex-col gap-2.5 items-center justify-center mb-7.5">
			<Handclap className="w-9 h-9" />
			<span data-testid="transaction-status" className="sr-only">
				{success ? 'Transaction Success' : 'Transaction Failed'}
			</span>
			{timestamp && <DateCard timestamp={Number(timestamp)} size="md" />}
		</div>
	);
}

export function ReceiptCard({ txn, activeAddress }: ReceiptCardProps) {
	const { events } = txn;
	const recognizedPackagesList = useRecognizedPackages();
	const summary = useTransactionSummary({
		transaction: txn,
		currentAddress: activeAddress,
		recognizedPackagesList,
	});

	const explorerHref = useExplorerLink({
		type: ExplorerLinkType.transaction,
		transactionID: summary?.digest,
	});
	if (!summary) return null;

	const stakedTxn = events?.find(({ type }) => type === '0x3::validator::StakingRequestEvent');

	const unstakeTxn = events?.find(({ type }) => type === '0x3::validator::UnstakingRequestEvent');

	// todo: re-using the existing staking cards for now
	if (stakedTxn || unstakeTxn) {
		return (
			<div className="block relative w-full h-full">
				<TransactionStatus success={summary?.status === 'success'} timestamp={txn.timestampMs} />
				{stakedTxn ? <StakeTxnCard event={stakedTxn} /> : null}
				{unstakeTxn ? <UnStakeTxnCard event={unstakeTxn} /> : null}
				<div className="mt-5">
					<GasSummary gasSummary={summary?.gas} />
				</div>
			</div>
		);
	}

	return (
		<div className="block relative w-full h-full">
			<TransactionStatus success={summary.status === 'success'} timestamp={txn.timestampMs} />
			<TransactionSummary showGasSummary summary={summary} />
			<a
				className="h-10 w-full no-underline flex items-center justify-center gap-1.25 bg-bfc-card rounded-lg text-body text-bfc-text2 font-medium"
				href={explorerHref!}
				target="_blank"
				rel="noreferrer"
			>
				View on Explorer
				<svg
					width="15"
					height="15"
					viewBox="0 0 15 15"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M9.01084 5.56946L3.9895 10.5902L4.81434 11.415L9.83508 6.39487V10.8195H11.0018V4.40279H4.58509V5.56946H9.01084Z"
						fill="#5A6070"
					/>
				</svg>
			</a>
			<div className="h-5 w-full"></div>
		</div>
	);
}
