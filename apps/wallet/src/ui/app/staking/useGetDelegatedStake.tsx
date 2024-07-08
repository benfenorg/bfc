// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { DelegatedStake } from '@benfen/bfc.js/client';
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery, type UseQueryResult } from '@tanstack/react-query';

export function useGetDelegatedStake(address: string): UseQueryResult<DelegatedStake[], Error> {
	const rpc = useBenfenClient();
	return useQuery({
		queryKey: ['validator', address],
		queryFn: () => rpc.getStakes({ owner: address }),
		staleTime: 10 * 1000,
		refetchInterval: 30 * 1000,
	});
}
