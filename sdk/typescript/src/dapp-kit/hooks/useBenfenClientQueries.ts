// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { UseQueryResult } from '@tanstack/react-query';
import { useQueries } from '@tanstack/react-query';

import { useBenfenClientContext } from './useBenfenClient.js';
import type { BenfenRpcMethods, UseBenfenClientQueryOptions } from './useBenfenClientQuery.js';

type BenfenClientQueryOptions = BenfenRpcMethods[keyof BenfenRpcMethods] extends infer Method
	? Method extends {
			name: infer M extends keyof BenfenRpcMethods;
			params?: infer P;
	  }
		? undefined extends P
			? {
					method: M;
					params?: P;
					options?: UseBenfenClientQueryOptions<M, unknown>;
			  }
			: {
					method: M;
					params: P;
					options?: UseBenfenClientQueryOptions<M, unknown>;
			  }
		: never
	: never;

export type UseBenfenClientQueriesResults<Args extends readonly BenfenClientQueryOptions[]> = {
	-readonly [K in keyof Args]: Args[K] extends {
		method: infer M extends keyof BenfenRpcMethods;
		readonly options?:
			| {
					select?: (...args: any[]) => infer R;
			  }
			| object;
	}
		? UseQueryResult<unknown extends R ? BenfenRpcMethods[M]['result'] : R>
		: never;
};

export function useBenfenClientQueries<
	const Queries extends readonly BenfenClientQueryOptions[],
	Results = UseBenfenClientQueriesResults<Queries>,
>({
	queries,
	combine,
}: {
	queries: Queries;
	combine?: (results: UseBenfenClientQueriesResults<Queries>) => Results;
}): Results {
	const benfenContext = useBenfenClientContext();

	return useQueries({
		combine: combine as never,
		queries: queries.map((query) => {
			const { method, params, options: { queryKey = [], ...restOptions } = {} } = query;

			return {
				...restOptions,
				queryKey: [benfenContext.network, method, params, ...queryKey],
				queryFn: async () => {
					return await benfenContext.client[method](params as never);
				},
			};
		}) as [],
	});
}
