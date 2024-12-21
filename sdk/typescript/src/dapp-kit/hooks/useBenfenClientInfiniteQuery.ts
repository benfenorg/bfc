// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type {
	InfiniteData,
	UseInfiniteQueryOptions,
	UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { BenfenClient } from '../../client/index.js';
import type { PartialBy } from '../types/utilityTypes.js';
import { useBenfenClientContext } from './useBenfenClient.js';

interface PaginatedResult {
	data?: unknown;
	nextCursor?: unknown;
	hasNextPage: boolean;
}

export type BenfenRpcPaginatedMethodName = {
	[K in keyof BenfenClient]: BenfenClient[K] extends (input: any) => Promise<PaginatedResult>
		? K
		: never;
}[keyof BenfenClient];

export type BenfenRpcPaginatedMethods = {
	[K in BenfenRpcPaginatedMethodName]: BenfenClient[K] extends (
		input: infer Params,
	) => Promise<
		infer Result extends { hasNextPage?: boolean | null; nextCursor?: infer Cursor | null }
	>
		? {
				name: K;
				result: Result;
				params: Params;
				cursor: Cursor;
		  }
		: never;
};

export type UseBenfenClientInfiniteQueryOptions<
	T extends keyof BenfenRpcPaginatedMethods,
	TData,
> = PartialBy<
	Omit<
		UseInfiniteQueryOptions<
			BenfenRpcPaginatedMethods[T]['result'],
			Error,
			TData,
			BenfenRpcPaginatedMethods[T]['result'],
			unknown[]
		>,
		'queryFn' | 'initialPageParam' | 'getNextPageParam'
	>,
	'queryKey'
>;

export function useBenfenClientInfiniteQuery<
	T extends keyof BenfenRpcPaginatedMethods,
	TData = InfiniteData<BenfenRpcPaginatedMethods[T]['result']>,
>(
	method: T,
	params: BenfenRpcPaginatedMethods[T]['params'],
	{
		queryKey = [],
		enabled = !!params,
		...options
	}: UseBenfenClientInfiniteQueryOptions<T, TData> = {},
): UseInfiniteQueryResult<TData, Error> {
	const benfenContext = useBenfenClientContext();

	return useInfiniteQuery({
		...options,
		initialPageParam: null,
		queryKey: [benfenContext.network, method, params, ...queryKey],
		enabled,
		queryFn: ({ pageParam }) =>
			benfenContext.client[method]({
				...(params ?? {}),
				cursor: pageParam,
			} as never),
		getNextPageParam: (lastPage) => (lastPage.hasNextPage ? lastPage.nextCursor ?? null : null),
	});
}
