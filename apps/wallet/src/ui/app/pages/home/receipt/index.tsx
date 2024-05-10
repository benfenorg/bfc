// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getExecutionStatusType } from '@benfen/bfc.js';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { Check24 } from '@mysten/icons';
import { useQuery } from '@tanstack/react-query';
import { useCallback, useMemo, useState } from 'react';
import { Navigate, useSearchParams, useNavigate, useLocation } from 'react-router-dom';

import Alert from '_components/alert';
import Loading from '_components/loading';
import Overlay from '_components/overlay';
import { ReceiptCard } from '_src/ui/app/components/receipt-card';
import { useActiveAddress } from '_src/ui/app/hooks/useActiveAddress';

function ReceiptPage() {
	const location = useLocation();
	const [searchParams] = useSearchParams();
	const [showModal, setShowModal] = useState(true);
	const activeAddress = useActiveAddress();

	// get tx results from url params
	const transactionId = searchParams.get('txdigest');
	const fromParam = searchParams.get('from');
	const rpc = useSuiClient();

	const { data, isLoading, isError } = useQuery({
		queryKey: ['transactions-by-id', transactionId],
		queryFn: async () => {
			return rpc.getTransactionBlock({
				digest: transactionId!,
				options: {
					showBalanceChanges: true,
					showObjectChanges: true,
					showInput: true,
					showEffects: true,
					showEvents: true,
				},
			});
		},
		enabled: !!transactionId,
		retry: 8,
		initialData: location.state?.response,
		retryDelay: (retryCount) => Math.min(1000 * 2 ** Math.floor(retryCount / 5), 30000),
	});

	const navigate = useNavigate();
	// return to previous route or from param if available
	const closeReceipt = useCallback(() => {
		fromParam ? navigate(`/${fromParam}`) : navigate(-1);
	}, [fromParam, navigate]);

	const pageTitle = useMemo(() => {
		if (data) {
			const executionStatus = getExecutionStatusType(data);

			// TODO: Infer out better name:
			const transferName = 'Transaction';

			return `${executionStatus === 'success' ? transferName : 'Transaction Failed'}`;
		}

		return 'Transaction Failed';
	}, [/*activeAddress,*/ data]);

	if (!transactionId || !activeAddress) {
		return <Navigate to="/transactions" replace={true} />;
	}

	return (
		<Loading loading={isLoading}>
			<Overlay
				showModal={showModal}
				setShowModal={setShowModal}
				title={pageTitle}
				closeOverlay={closeReceipt}
				closeIcon={<Check24 className="text-white w-6 h-6" />}
			>
				{isError ? (
					<div className="mb-2 h-fit">
						<Alert>Something went wrong</Alert>
					</div>
				) : null}

				{data && <ReceiptCard txn={data} activeAddress={activeAddress} />}
			</Overlay>
		</Loading>
	);
}

export default ReceiptPage;
