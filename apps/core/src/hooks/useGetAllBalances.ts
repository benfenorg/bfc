// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { CoinBalance } from '@benfen/bfc.js/client';
import { useQuery } from '@tanstack/react-query';

import { useRpcClient } from '../api/RpcClientContext';

export function useGetAllBalances<TResult = CoinBalance[]>(
	address?: string | null,
	refetchInterval?: number,
	staleTime?: number,
	select?: (data: CoinBalance[]) => TResult,
) {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['get-all-balance', address],
		queryFn: () => rpc.getAllBalances({ owner: address! }),
		enabled: !!address,
		refetchInterval,
		staleTime,
		select,
	});
}
