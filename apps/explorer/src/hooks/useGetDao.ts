// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useRpcClient } from '@mysten/core';
import { useWalletKit } from '@mysten/wallet-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetDao() {
	const rpc = useRpcClient();
	const { currentAccount } = useWalletKit();
	return useQuery({
		queryKey: ['dao', 'inner', currentAccount],
		enabled: Boolean(currentAccount?.address),
		queryFn: () => rpc.getInnerDao(),
	});
}
