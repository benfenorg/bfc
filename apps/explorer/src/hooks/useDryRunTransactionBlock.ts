// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { type TransactionBlock } from '@benfen/bfc.js/transactions';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { useWalletKit } from '@benfen/wallet-kit';
import { useCallback } from 'react';

export const useDryRunTransactionBlock = () => {
	const { currentAccount } = useWalletKit();
	const client = useSuiClient();

	const dryRun = useCallback(
		async (tx: TransactionBlock) => {
			if (!currentAccount?.address) {
				throw new Error('Not Connected');
			}
			tx.setSenderIfNotSet(currentAccount.address);
			const result = await client.dryRunTransactionBlock({
				transactionBlock: await tx.build({ client }),
			});
			if (result.effects.status.status !== 'success') {
				throw new Error(
					`Dry run failed, could not automatically determine a budget: ${result.effects.status.error}`,
				);
			}
		},
		[client, currentAccount],
	);

	return dryRun;
};
