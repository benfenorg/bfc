// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetReferenceGasPrice() {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['current-epoch', 'gas-price'],
		queryFn: () => client.getReferenceGasPrice(),
	});
}
