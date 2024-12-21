// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { UseQueryOptions, UseQueryResult } from '@tanstack/react-query';

import type { ResolvedNameServiceNames } from '../../client/index.js';
import { useBenfenClientQuery } from './useBenfenClientQuery.js';

export function useResolveBenfenNSName(
	address?: string | null,
	options?: Omit<
		UseQueryOptions<ResolvedNameServiceNames, Error, string | null, unknown[]>,
		'queryFn' | 'queryKey' | 'select'
	>,
): UseQueryResult<string | null, Error> {
	return useBenfenClientQuery(
		'resolveNameServiceNames',
		{
			address: address!,
			limit: 1,
		},
		{
			...options,
			refetchOnWindowFocus: false,
			retry: false,
			select: (data) => (data.data.length > 0 ? data.data[0] : null),
			enabled: !!address && options?.enabled !== false,
		},
	);
}
