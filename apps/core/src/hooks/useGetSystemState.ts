// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';

import { useRpcClient } from '../api/RpcClientContext';

export function useGetSystemState() {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['system', 'state'],
		queryFn: () => rpc.getLatestBenfeSystemState(),
	});
}
