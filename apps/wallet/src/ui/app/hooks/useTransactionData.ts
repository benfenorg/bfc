// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { Transaction } from '@benfen/bfc.js/transactions';
import { BFC_TYPE_ARG } from '@benfen/bfc.js/utils';
import { useFormatCoin } from '@mysten/core';
import { useQuery } from '@tanstack/react-query';

export function useTransactionData(sender?: string | null, transaction?: Transaction | null) {
	const client = useBenfenClient();
	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['transaction-data', transaction?.serialize()],
		queryFn: async () => {
			const clonedTransaction = Transaction.from(transaction!);
			if (sender) {
				clonedTransaction.setSenderIfNotSet(sender);
			}
			// Build the transaction to bytes, which will ensure that the transaction data is fully populated:
			await clonedTransaction!.build({ client });
			return clonedTransaction!.blockData;
		},
		enabled: !!transaction,
	});
}

export function useTransactionGasBudget(sender?: string | null, transaction?: Transaction | null) {
	const { data, ...rest } = useTransactionData(sender, transaction);

	const [formattedGas] = useFormatCoin(data?.gasConfig.budget, BFC_TYPE_ARG);

	return {
		data: formattedGas,
		...rest,
	};
}
