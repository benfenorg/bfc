// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
<<<<<<< HEAD
import { QredoTransaction } from './QredoTransaction';
=======

>>>>>>> mainnet-v1.24.1
import { ErrorBoundary } from '_components/error-boundary';
import Loading from '_components/loading';
import { NoActivityCard } from '_components/transactions-card/NoActivityCard';
import { AccountType } from '_src/background/keyring/Account';
import { type TransactionStatus } from '_src/shared/qredo-api';
import Alert from '_src/ui/app/components/alert';
import { useActiveAccount } from '_src/ui/app/hooks/useActiveAccount';
import { useActiveAddress } from '_src/ui/app/hooks/useActiveAddress';
import { useGetQredoTransactions } from '_src/ui/app/hooks/useGetQredoTransactions';

import { QredoTransaction } from './QredoTransaction';

const PENDING_QREDO_TRANSACTION_STATUSES: TransactionStatus[] = [
	'approved',
	'authorized',
	'created',
	'pending',
	'pushed',
	'scheduled',
	'signed',
];

export function QredoPendingTransactions() {
	const activeAddress = useActiveAddress();
	const activeAccount = useActiveAccount();
	const isQredoAccount = activeAccount?.type === AccountType.QREDO;
	const qredoID = isQredoAccount ? activeAccount.qredoConnectionID : undefined;
	const {
		data: qredoTransactions,
		isPending,
		error,
	} = useGetQredoTransactions({
		qredoID,
		filterStatus: PENDING_QREDO_TRANSACTION_STATUSES,
	});
	if (error) {
		return <Alert>{(error as Error)?.message}</Alert>;
	}
	return (
		<Loading loading={isPending}>
			{qredoTransactions?.length && activeAddress ? (
				qredoTransactions.map((txn) => (
					<ErrorBoundary key={txn.txID}>
						<QredoTransaction qredoID={qredoID} qredoTransactionID={txn.txID} />
					</ErrorBoundary>
				))
			) : (
				<NoActivityCard message="When available, pending Qredo transactions will show up here." />
			)}
		</Loading>
	);
}
