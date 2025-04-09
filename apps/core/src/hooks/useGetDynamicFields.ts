// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { DynamicFieldPage } from '@benfen/bfc.js/client';
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { normalizeHexAddress } from '@benfen/bfc.js/utils';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_PAGE_SIZE = 10;

export function useGetDynamicFields(parentId: string, maxPageSize = MAX_PAGE_SIZE) {
	const client = useBenfenClient();
	return useInfiniteQuery<DynamicFieldPage>({
		queryKey: ['dynamic-fields', { maxPageSize, parentId }],
		queryFn: ({ pageParam = null }) =>
			client.getDynamicFields({
				parentId: normalizeHexAddress(parentId),
				cursor: pageParam as string | null,
				limit: maxPageSize,
			}),
		enabled: !!parentId,
		initialPageParam: null,
		getNextPageParam: ({ nextCursor, hasNextPage }) => (hasNextPage ? nextCursor : null),
	});
}
