// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useSuiClient } from '@mysten/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetDao() {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['dao', 'inner'],
		queryFn: () => client.getInnerDao(),
	});
}
