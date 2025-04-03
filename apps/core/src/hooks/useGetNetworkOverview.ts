// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetNetworkOverview() {
	const client = useBenfenClient();
	return useQuery({
		queryKey: ['home', 'overview'],
		queryFn: () => client.getNetworkOverview(),
	});
}
