// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useInfiniteQuery } from '@tanstack/react-query';
import { normalizeSuiAddress } from '@benfen/bfc.js/utils';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
=======
import { useSuiClient } from '@mysten/dapp-kit';
import { DynamicFieldPage } from '@mysten/sui.js/client';
import { normalizeSuiAddress } from '@mysten/sui.js/utils';
import { useInfiniteQuery } from '@tanstack/react-query';
>>>>>>> mainnet-v1.24.1

const MAX_PAGE_SIZE = 10;

export function useGetDynamicFields(parentId: string, maxPageSize = MAX_PAGE_SIZE) {
	const client = useSuiClient();
	return useInfiniteQuery<DynamicFieldPage>({
		queryKey: ['dynamic-fields', { maxPageSize, parentId }],
		queryFn: ({ pageParam = null }) =>
			client.getDynamicFields({
				parentId: normalizeSuiAddress(parentId),
				cursor: pageParam as string | null,
				limit: maxPageSize,
			}),
		enabled: !!parentId,
		initialPageParam: null,
		getNextPageParam: ({ nextCursor, hasNextPage }) => (hasNextPage ? nextCursor : null),
	});
}
