// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useSuiClient } from '@mysten/dapp-kit';
import { ArrowRight12 } from '@mysten/icons';
import { Text } from '@mysten/ui';
import { useQuery } from '@tanstack/react-query';
import { useEffect, useRef, useState } from 'react';

import { genTableDataFromTxData } from '../transactions/TxCardUtils';
import { useGetTransactionBlocks } from '~/hooks/useGetTransactionBlocks';
import { Link } from '~/ui/Link';
import { Pagination, useCursorPagination } from '~/ui/Pagination';
import { PlaceholderTable } from '~/ui/PlaceholderTable';
import { TableCard } from '~/ui/TableCard';
import { numberSuffix } from '~/utils/numberUtil';

const DEFAULT_TRANSACTIONS_LIMIT = 20;

interface Props {
	disablePagination?: boolean;
	refetchInterval?: number;
	initialLimit?: number;
	transactionKindFilter?: 'ProgrammableTransaction';
}

export function TransactionsActivityTable({
	disablePagination,
<<<<<<< HEAD
=======
	refetchInterval,
>>>>>>> heads/mainnet-v1.9.1
	initialLimit = DEFAULT_TRANSACTIONS_LIMIT,
	transactionKindFilter,
}: Props) {
	const [limit, setLimit] = useState(initialLimit);
<<<<<<< HEAD
	const rpc = useRpcClient();
	const { data: count } = useQuery({
		queryKey: ['transactions', 'count'],
		queryFn: () => rpc.getTotalTransactionBlocks(),
=======
	const client = useSuiClient();
	const { data: count } = useQuery({
		queryKey: ['transactions', 'count'],
		queryFn: () => client.getTotalTransactionBlocks(),
>>>>>>> heads/mainnet-v1.9.1
		cacheTime: 24 * 60 * 60 * 1000,
		staleTime: Infinity,
		retry: false,
	});
	const transactions = useGetTransactionBlocks(
		transactionKindFilter ? { TransactionKind: transactionKindFilter } : undefined,
		limit,
<<<<<<< HEAD
=======
		refetchInterval,
>>>>>>> heads/mainnet-v1.9.1
	);
	const { data, isFetching, pagination, isLoading, isError } = useCursorPagination(transactions);
	const goToFirstPageRef = useRef(pagination.onFirst);
	goToFirstPageRef.current = pagination.onFirst;
	const cardData = data ? genTableDataFromTxData(data.data) : undefined;

	useEffect(() => {
		goToFirstPageRef.current();
	}, [transactionKindFilter]);
	return (
<<<<<<< HEAD
		<div data-testid="tx" className="obc-table-container">
			{isError && (
				<div className="px-3.5 pt-2 font-sans font-semibold text-issue-dark">
=======
		<div data-testid="tx">
			{isError && (
				<div className="pt-2 font-sans font-semibold text-issue-dark">
>>>>>>> heads/mainnet-v1.9.1
					Failed to load Transactions
				</div>
			)}
			<div className="flex flex-col space-y-3 text-left">
				{isLoading || isFetching || !cardData ? (
					<PlaceholderTable
						rowCount={limit}
						rowHeight="16px"
						colHeadings={['Digest', 'Sender', 'Txns', 'Gas', 'Time']}
						colWidths={['30%', '30%', '10%', '20%', '10%']}
					/>
				) : (
					<div>
						<TableCard data={cardData.data} columns={cardData.columns} />
					</div>
				)}

<<<<<<< HEAD
				<div className="flex justify-between bg-obc-card p-3.5">
=======
				<div className="flex justify-between">
>>>>>>> heads/mainnet-v1.9.1
					{!disablePagination ? (
						<Pagination {...pagination} />
					) : (
						<Link to="/recent" after={<ArrowRight12 className="h-3 w-3 -rotate-45" />}>
							View all
						</Link>
					)}

					<div className="flex items-center space-x-3">
<<<<<<< HEAD
						<Text variant="body/normal" color="steel-darker">
=======
						<Text variant="body/medium" color="steel-dark">
>>>>>>> heads/mainnet-v1.9.1
							{count ? numberSuffix(Number(count)) : '-'}
							{` Total`}
						</Text>
						{!disablePagination && (
							<select
<<<<<<< HEAD
								className="obc-form-select form-select rounded-md border border-obc-border px-3 py-2 pr-8 text-bodySmall font-normal leading-[1.2] text-steel-darker shadow-button"
=======
								className="form-select rounded-md border border-gray-45 px-3 py-2 pr-8 text-bodySmall font-medium leading-[1.2] text-steel-dark shadow-button"
>>>>>>> heads/mainnet-v1.9.1
								value={limit}
								onChange={(e) => {
									setLimit(Number(e.target.value));
									pagination.onFirst();
								}}
							>
								<option value={20}>20 Per Page</option>
								<option value={40}>40 Per Page</option>
								<option value={60}>60 Per Page</option>
							</select>
						)}
					</div>
				</div>
			</div>
		</div>
	);
}
