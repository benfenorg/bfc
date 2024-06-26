// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';

import { useRpcClient } from '../api/RpcClientContext';

export function useGetLatestCheckpointSequenceNumber() {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['checkpoints', 'count'],
		queryFn: () => rpc.getLatestCheckpointSequenceNumber(),
	});
}
