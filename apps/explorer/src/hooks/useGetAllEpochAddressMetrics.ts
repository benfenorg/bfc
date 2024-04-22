// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiClient } from '@benfen/bfc.js/client';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetAllEpochAddressMetrics(
	...input: Parameters<SuiClient['getAllEpochAddressMetrics']>
) {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['get', 'all', 'epoch', 'addresses', ...input],
		queryFn: () => client.getAllEpochAddressMetrics(...input),
	});
}
