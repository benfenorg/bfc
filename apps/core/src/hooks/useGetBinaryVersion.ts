// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';

import { useRpcClient } from '../api/RpcClientContext';

// Current API version is the same as the binary version
export function useGetBinaryVersion() {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['binary-version'],
		queryFn: () => rpc.getRpcApiVersion(),
	});
}
