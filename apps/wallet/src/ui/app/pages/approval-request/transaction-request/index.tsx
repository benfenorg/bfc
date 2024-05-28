// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { TransactionBlock } from '@benfen/bfc.js/transactions';
import { useTransactionSummary } from '@mysten/core';
import { useMemo, useState } from 'react';

import { GasFees } from './GasFees';
import { TransactionDetails } from './TransactionDetails';
import { ConfirmationModal } from '../../../shared/ConfirmationModal';
=======
// import { Transaction } from '@mysten/sui.js';
>>>>>>> mainnet-v1.24.1
import { UserApproveContainer } from '_components/user-approve-container';
import { useAppDispatch, useSigner, useTransactionData, useTransactionDryRun } from '_hooks';
import { type TransactionApprovalRequest } from '_payloads/transactions/ApprovalRequest';
import { respondToTransactionRequest } from '_redux/slices/transaction-requests';
import { ampli } from '_src/shared/analytics/ampli';
import { useQredoTransaction } from '_src/ui/app/hooks/useQredoTransaction';
import { useRecognizedPackages } from '_src/ui/app/hooks/useRecognizedPackages';
import { PageMainLayoutTitle } from '_src/ui/app/shared/page-main-layout/PageMainLayoutTitle';
import { TransactionSummary } from '_src/ui/app/shared/transaction-summary';
import { useTransactionSummary } from '@mysten/core';
import { TransactionBlock } from '@mysten/sui.js/transactions';
import { useMemo, useState } from 'react';

import { ConfirmationModal } from '../../../shared/ConfirmationModal';
import { GasFees } from './GasFees';
import { TransactionDetails } from './TransactionDetails';

export type TransactionRequestProps = {
	txRequest: TransactionApprovalRequest;
};

// Some applications require *a lot* of transactions to interact with, and this
// eats up our analytics event quota. As a short-term solution so we don't have
// to stop tracking this event entirely, we'll just manually exclude application
// origins with this list
const appOriginsToExcludeFromAnalytics = ['https://sui8192.ethoswallet.xyz'];

export function TransactionRequest({ txRequest }: TransactionRequestProps) {
	const addressForTransaction = txRequest.tx.account;
	const signer = useSigner(addressForTransaction);
	const dispatch = useAppDispatch();
	const transaction = useMemo(() => {
		const tx = TransactionBlock.from(txRequest.tx.data);
		if (addressForTransaction) {
			tx.setSenderIfNotSet(addressForTransaction);
		}
		return tx;
	}, [txRequest.tx.data, addressForTransaction]);
	const { isPending, isError } = useTransactionData(addressForTransaction, transaction);
	const [isConfirmationVisible, setConfirmationVisible] = useState(false);

	const {
		data,
		isError: isDryRunError,
		isPending: isDryRunLoading,
	} = useTransactionDryRun(addressForTransaction, transaction);
	const recognizedPackagesList = useRecognizedPackages();

	const summary = useTransactionSummary({
		transaction: data,
		currentAddress: addressForTransaction,
		recognizedPackagesList,
	});
	const { clientIdentifier, notificationModal } = useQredoTransaction(true);
	if (!signer) {
		return null;
	}
	return (
		<>
			<UserApproveContainer
				origin={txRequest.origin}
				originFavIcon={txRequest.originFavIcon}
				approveTitle="Approve"
				rejectTitle="Reject"
				onSubmit={async (approved: boolean) => {
					if (isPending) return;
					if (approved && isError) {
						setConfirmationVisible(true);
						return;
					}
					await dispatch(
						respondToTransactionRequest({
							approved,
							txRequestID: txRequest.id,
							signer,
							clientIdentifier,
						}),
					);
					if (!appOriginsToExcludeFromAnalytics.includes(txRequest.origin)) {
						ampli.respondedToTransactionRequest({
							applicationUrl: txRequest.origin,
							approvedTransaction: approved,
							receivedFailureWarning: false,
						});
					}
				}}
				address={addressForTransaction}
				approveLoading={isPending || isConfirmationVisible}
				checkAccountLock
			>
				<PageMainLayoutTitle title="Approve Transaction" />
<<<<<<< HEAD

				<div className="flex flex-col gap-5">
					<TransactionSummary
						isDryRun
						isLoading={isDryRunLoading}
						isError={isDryRunError}
						showGasSummary={false}
						summary={summary}
					/>
				</div>
				<section className="flex flex-col gap-5">
					<GasFees sender={addressForTransaction} transaction={transaction} />
					<TransactionDetails sender={addressForTransaction} transaction={transaction} />
				</section>
=======
				<div className="flex flex-col">
					<div className="flex flex-col gap-4">
						<TransactionSummary
							isDryRun
							isLoading={isDryRunLoading}
							isError={isDryRunError}
							showGasSummary={false}
							summary={summary}
						/>
					</div>
					<section className=" bg-white -mx-6">
						<div className="flex flex-col gap-4 p-6">
							<GasFees sender={addressForTransaction} transaction={transaction} />
							<TransactionDetails sender={addressForTransaction} transaction={transaction} />
						</div>
					</section>
				</div>
>>>>>>> mainnet-v1.24.1
			</UserApproveContainer>
			<ConfirmationModal
				isOpen={isConfirmationVisible}
				title="This transaction might fail. Are you sure you still want to approve the transaction?"
				hint="You will still be charged a gas fee for this transaction."
				confirmStyle="primary"
				confirmText="Approve"
				cancelText="Reject"
				cancelStyle="warning"
				onResponse={async (isConfirmed) => {
					await dispatch(
						respondToTransactionRequest({
							approved: isConfirmed,
							txRequestID: txRequest.id,
							signer,
							clientIdentifier,
						}),
					);
					ampli.respondedToTransactionRequest({
						applicationUrl: txRequest.origin,
						approvedTransaction: isConfirmed,
						receivedFailureWarning: true,
					});
					setConfirmationVisible(false);
				}}
			/>
			{notificationModal}
		</>
	);
}
