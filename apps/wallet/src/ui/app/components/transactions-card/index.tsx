// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
<<<<<<< HEAD
import {
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
	getTransactionKindName,
	getTransactionKind,
	getTransactionSender,
} from '@benfen/bfc.js';
import { useTransactionSummary, getLabel } from '@mysten/core';
import { Link } from 'react-router-dom';

import { TxnTypeLabel } from './TxnActionLabel';
import { TxnIcon } from './TxnIcon';
=======

>>>>>>> mainnet-v1.24.1
import { DateCard } from '_app/shared/date-card';
import { Text } from '_app/shared/text';
import { useGetTxnRecipientAddress } from '_hooks';
import { useRecognizedPackages } from '_src/ui/app/hooks/useRecognizedPackages';
<<<<<<< HEAD

import type { SuiTransactionBlockResponse } from '@benfen/bfc.js/client';
=======
import { getLabel, useTransactionSummary } from '@mysten/core';
import type { SuiTransactionBlockResponse } from '@mysten/sui.js/client';
import { Link } from 'react-router-dom';

import { TxnTypeLabel } from './TxnActionLabel';
import { TxnIcon } from './TxnIcon';
>>>>>>> mainnet-v1.24.1

export function TransactionCard({
	txn,
	address,
}: {
	txn: SuiTransactionBlockResponse;
	address: string;
}) {
	const transaction = getTransactionKind(txn)!;
	const executionStatus = getExecutionStatusType(txn);
	getTransactionKindName(transaction);

	const recognizedPackagesList = useRecognizedPackages();

	const summary = useTransactionSummary({
		transaction: txn,
		currentAddress: address,
		recognizedPackagesList,
	});

	// we only show Sui Transfer amount or the first non-Sui transfer amount

	const recipientAddress = useGetTxnRecipientAddress({ txn, address });

	const isSender = address === getTransactionSender(txn);

	const error = getExecutionStatusError(txn);

	// Transition label - depending on the transaction type and amount
	// Epoch change without amount is delegation object
	// Special case for staking and unstaking move call transaction,
	// For other transaction show Sent or Received

	// TODO: Support programmable tx:
	// Show sui symbol only if transfer transferAmount coinType is SUI_TYPE_ARG, staking or unstaking
	const showSuiSymbol = false;

	const timestamp = txn.timestampMs;

	return (
		<Link
			data-testid="link-to-txn"
			to={`/receipt?${new URLSearchParams({
				txdigest: getTransactionDigest(txn),
			}).toString()}`}
			className="flex items-center w-full gap-1.25 p-2.5 no-underline"
		>
			<div className="w-6 h-6">
				<TxnIcon
					txnFailed={executionStatus !== 'success' || !!error}
					// TODO: Support programmable transactions variable icons here:
					variant={getLabel(txn, address)}
				/>
			</div>
			<div className="flex flex-col w-full">
				{error ? (
					<div className="flex w-full justify-between">
						<div className="flex flex-col w-full gap-1.5">
							<Text color="bfc-text1" weight="medium">
								Transaction Failed
							</Text>

							<div className="flex break-all">
								<Text variant="body" weight="medium" color="bfc-text1">
									{error}
								</Text>
							</div>
						</div>
						{/* {transferAmountComponent} */}
					</div>
				) : (
					<>
						<div className="flex w-full justify-between">
							<div className="flex gap-1 align-middle items-baseline">
								<Text color="bfc-text1" weight="medium" variant="body">
									{summary?.label}
								</Text>
								{showSuiSymbol && (
									<Text color="bfc-text1" weight="medium" variant="body">
										BFC
									</Text>
								)}
							</div>
							{/* {transferAmountComponent} */}
						</div>

						{/* TODO: Support programmable tx: */}
						<TxnTypeLabel address={recipientAddress!} isSender={isSender} isTransfer={false} />
						{/* {objectId && <TxnImage id={objectId} />} */}
					</>
				)}

				{timestamp && <DateCard timestamp={Number(timestamp)} size="sm" />}
			</div>
		</Link>
	);
}
