// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { X12, Dot12 } from '@mysten/icons';
import {
	getExecutionStatusType,
	getTotalGasUsed,
	getTransactionSender,
	type SuiTransactionBlockResponse,
} from '@mysten/sui.js';
import { type SuiClient } from '@mysten/sui.js/client';

import { SuiAmount } from '../Table/SuiAmount';
import { TxTimeType } from '../tx-time/TxTimeType';
import { AddressLink, TransactionLink } from '~/ui/InternalLink';

// Generate table data from the transaction data
export const genTableDataFromTxData = (results: SuiTransactionBlockResponse[]) => ({
	data: results.map((transaction) => {
		const status = getExecutionStatusType(transaction);
		const sender = getTransactionSender(transaction);

		return {
			date: <TxTimeType timestamp={Number(transaction.timestampMs || 0)} />,
			digest: (
				<TransactionLink
					digest={transaction.digest}
					before={
						status === 'success' ? (
							<Dot12 className="text-success" />
						) : (
							<X12 className="text-issue-dark" />
						)
					}
				/>
			),
			txns: (
				<div>
					{transaction.transaction?.data.transaction.kind === 'ProgrammableTransaction'
						? transaction.transaction.data.transaction.transactions.length
						: '--'}
				</div>
			),
			gas: <SuiAmount amount={getTotalGasUsed(transaction)} />,
			sender: sender ? <AddressLink address={sender} /> : '-',
		};
	}),
	columns: [
		{
			header: 'Digest',
			accessorKey: 'digest',
		},
		{
			header: 'Sender',
			accessorKey: 'sender',
		},
		{
			header: 'Txns',
			accessorKey: 'txns',
		},
		{
			header: 'Gas',
			accessorKey: 'gas',
		},
		{
			header: 'Time',
			accessorKey: 'date',
		},
	],
});

const dedupe = (arr: string[]) => Array.from(new Set(arr));

export const getDataOnTxDigests = (client: SuiClient, transactions: string[]) =>
	client
		.multiGetTransactionBlocks({
			digests: dedupe(transactions),
			options: {
				showInput: true,
				showEffects: true,
				showEvents: true,
			},
		})
		.then((transactions) =>
			// Remove failed transactions
			transactions.filter((item) => item),
		);
