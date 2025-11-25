// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { type Transaction } from '@benfen/bfc.js/transactions';
import { useCallback } from 'react';

import { useActiveAccount } from './useActiveAccount';

export const useDryRunTransaction = () => {
	const client = useBenfenClient();
	const activeAccount = useActiveAccount();

	const dryRun = useCallback(
		async (tx: Transaction) => {
			if (!activeAccount?.address) {
				throw new Error('Not Connected');
			}
			tx.setSenderIfNotSet(activeAccount.address);
			console.log('useDryRunTransaction::dryRun::transaction', tx);
			const result = await client.dryRunTransactionBlock({
				transactionBlock: await tx.build({ client }),
			});
			if (result.effects.status.status !== 'success') {
				throw new Error(
					`Dry run failed, could not automatically determine a budget: ${result.effects.status.error}`,
				);
			}
			return result;
		},
		[client, activeAccount],
	);

	return dryRun;
};
