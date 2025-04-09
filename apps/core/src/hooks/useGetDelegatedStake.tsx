// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { DelegatedStake } from '@benfen/bfc.js/client';
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery, type UseQueryOptions } from '@tanstack/react-query';

type UseGetDelegatedStakesOptions = {
	address: string;
} & Omit<UseQueryOptions<DelegatedStake[], Error>, 'queryKey' | 'queryFn'>;

export function useGetDelegatedStake(options: UseGetDelegatedStakesOptions) {
	const client = useBenfenClient();
	const { address, ...queryOptions } = options;

	return useQuery({
		queryKey: ['delegated-stakes', address],
		queryFn: () => client.getStakes({ owner: address }),
		...queryOptions,
	});
}
