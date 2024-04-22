// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';

export function useGetCoinBalance(
	coinType: string,
	address?: string | null,
	refetchInterval?: number,
	staleTime?: number,
) {
	const rpc = useSuiClient();

	return useQuery({
		queryKey: ['coin-balance', address, coinType],
		queryFn: () => rpc.getBalance({ owner: address!, coinType }),
		enabled: !!address && !!coinType,
		refetchInterval,
		staleTime,
	});
}
