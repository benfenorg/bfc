// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useSuiClient } from '@mysten/dapp-kit';
import { useWalletKit } from '@mysten/wallet-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetDao() {
	const client = useSuiClient();
	const { currentAccount } = useWalletKit();
	return useQuery({
		queryKey: ['dao', 'inner', currentAccount],
		enabled: Boolean(currentAccount?.address),
		queryFn: () => client.getInnerDao(),
	});
}
