// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { PaginatedCoins } from '@benfen/bfc.js/client';
import { UseInfiniteQueryResult, useInfiniteQuery } from '@tanstack/react-query';
=======
import { useSuiClient } from '@mysten/dapp-kit';
import { PaginatedCoins } from '@mysten/sui.js/client';
import { useInfiniteQuery } from '@tanstack/react-query';
>>>>>>> mainnet-v1.24.1

const MAX_COINS_PER_REQUEST = 10;

export function useGetCoins(
	coinType: string,
	address?: string | null,
	maxCoinsPerRequest = MAX_COINS_PER_REQUEST,
) {
	const client = useSuiClient();
	return useInfiniteQuery<PaginatedCoins>({
		queryKey: ['get-coins', address, coinType, maxCoinsPerRequest],
		initialPageParam: null,
		getNextPageParam: ({ hasNextPage, nextCursor }) => (hasNextPage ? nextCursor : null),
		queryFn: ({ pageParam }) =>
			client.getCoins({
				owner: address!,
				coinType,
				cursor: pageParam as string | null,
				limit: maxCoinsPerRequest,
			}),
		enabled: !!address,
	});
}
