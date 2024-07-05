// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { UseQueryOptions, UseQueryResult } from '@tanstack/react-query';
import { useQuery } from '@tanstack/react-query';

import type { BenfenClient } from '../../client/index.js';
import type { PartialBy } from '../types/utilityTypes.js';
import { useBenfenClientContext } from './useBenfenClient.js';

export type BenfenRpcMethodName = {
	[K in keyof BenfenClient]: BenfenClient[K] extends
		| ((input: any) => Promise<any>)
		| (() => Promise<any>)
		? K
		: never;
}[keyof BenfenClient];

export type BenfenRpcMethods = {
	[K in BenfenRpcMethodName]: BenfenClient[K] extends (input: infer P) => Promise<infer R>
		? {
				name: K;
				result: R;
				params: P;
		  }
		: BenfenClient[K] extends () => Promise<infer R>
		? {
				name: K;
				result: R;
				params: undefined | object;
		  }
		: never;
};

export type UseBenfenClientQueryOptions<T extends keyof BenfenRpcMethods, TData> = PartialBy<
	Omit<UseQueryOptions<BenfenRpcMethods[T]['result'], Error, TData, unknown[]>, 'queryFn'>,
	'queryKey'
>;

export function useBenfenClientQuery<
	T extends keyof BenfenRpcMethods,
	TData = BenfenRpcMethods[T]['result'],
>(
	...args: undefined extends BenfenRpcMethods[T]['params']
		? [
				method: T,
				params?: BenfenRpcMethods[T]['params'],
				options?: UseBenfenClientQueryOptions<T, TData>,
		  ]
		: [
				method: T,
				params: BenfenRpcMethods[T]['params'],
				options?: UseBenfenClientQueryOptions<T, TData>,
		  ]
): UseQueryResult<TData, Error> {
	const [method, params, { queryKey = [], ...options } = {}] = args as [
		method: T,
		params?: BenfenRpcMethods[T]['params'],
		options?: UseBenfenClientQueryOptions<T, TData>,
	];

	const benfenContext = useBenfenClientContext();

	return useQuery({
		...options,
		queryKey: [benfenContext.network, method, params, ...queryKey],
		queryFn: async () => {
			return await benfenContext.client[method](params as never);
		},
	});
}
