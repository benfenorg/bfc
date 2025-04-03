// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type BenfenTransactionBlockResponse } from '@benfen/bfc.js/client';
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useQueryTransactionsByAddress(address: string | null) {
	const rpc = useBenfenClient();

	return useQuery({
		queryKey: ['transactions-by-address', address],
		queryFn: async () => {
			// combine from and to transactions
			const [txnIds, fromTxnIds] = await Promise.all([
				rpc.queryTransactionBlocks({
					filter: {
						ToAddress: address!,
					},
					options: {
						showInput: true,
						showEffects: true,
						showEvents: true,
					},
				}),
				rpc.queryTransactionBlocks({
					filter: {
						FromAddress: address!,
					},
					options: {
						showInput: true,
						showEffects: true,
						showEvents: true,
					},
				}),
			]);

			const inserted = new Map();
			const uniqueList: BenfenTransactionBlockResponse[] = [];

			[...txnIds.data, ...fromTxnIds.data]
				.sort((a, b) => Number(b.timestampMs ?? 0) - Number(a.timestampMs ?? 0))
				.forEach((txb) => {
					if (inserted.get(txb.digest)) return;
					uniqueList.push(txb);
					inserted.set(txb.digest, true);
				});

			return uniqueList;
		},
		enabled: !!address,
		staleTime: 10 * 1000,
		refetchInterval: 60 * 1000,
	});
}
