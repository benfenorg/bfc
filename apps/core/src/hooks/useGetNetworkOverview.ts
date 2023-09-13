// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '../api/RpcClientContext';
import { useQuery } from '@tanstack/react-query';

export function useGetNetworkOverview() {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['home', 'overview'],
		queryFn: () => rpc.getNetworkOverview(),
	});
}
