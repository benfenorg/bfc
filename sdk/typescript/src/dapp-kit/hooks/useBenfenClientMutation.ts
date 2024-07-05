// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';

import { useBenfenClientContext } from './useBenfenClient.js';
import type { BenfenRpcMethods } from './useBenfenClientQuery.js';

export type UseBenfenClientMutationOptions<T extends keyof BenfenRpcMethods> = Omit<
	UseMutationOptions<
		BenfenRpcMethods[T]['result'],
		Error,
		BenfenRpcMethods[T]['params'],
		unknown[]
	>,
	'mutationFn'
>;

export function useBenfenClientMutation<T extends keyof BenfenRpcMethods>(
	method: T,
	options: UseBenfenClientMutationOptions<T> = {},
): UseMutationResult<
	BenfenRpcMethods[T]['result'],
	Error,
	BenfenRpcMethods[T]['params'],
	unknown[]
> {
	const benfenContext = useBenfenClientContext();

	return useMutation({
		...options,
		mutationFn: async (params) => {
			return await benfenContext.client[method](params as never);
		},
	});
}
