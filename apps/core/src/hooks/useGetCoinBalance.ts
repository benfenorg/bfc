// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetCoinBalance(
	coinType: string,
	address?: string | null,
	refetchInterval?: number,
	staleTime?: number,
) {
	const rpc = useBenfenClient();

	return useQuery({
		queryKey: ['coin-balance', address, coinType],
		queryFn: () => rpc.getBalance({ owner: address!, coinType }),
		enabled: !!address && !!coinType,
		refetchInterval,
		staleTime,
	});
}
