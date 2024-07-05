// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

const defaultOptions = {
	cacheTime: 24 * 60 * 60 * 1000,
	staleTime: Infinity,
	retry: 5,
};
export function useGetTotalTransactionBlocks(options = defaultOptions) {
	const client = useBenfenClient();
	return useQuery({
		queryKey: ['home', 'transaction-count'],
		queryFn: () => client.getTotalTransactionBlocks(),
		...options,
	});
}
