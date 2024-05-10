// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useInfiniteQuery } from '@tanstack/react-query';
import { normalizeSuiAddress } from '@benfen/bfc.js/utils';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';

const MAX_PAGE_SIZE = 10;

export function useGetDynamicFields(parentId: string, maxPageSize = MAX_PAGE_SIZE) {
	const client = useSuiClient();
	return useInfiniteQuery(
		['dynamic-fields', parentId],
		({ pageParam = null }) =>
			client.getDynamicFields({
				parentId: normalizeSuiAddress(parentId),
				cursor: pageParam,
				limit: maxPageSize,
			}),
		{
			enabled: !!parentId,
			getNextPageParam: ({ nextCursor, hasNextPage }) => (hasNextPage ? nextCursor : null),
		},
	);
}
