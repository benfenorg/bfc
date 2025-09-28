// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

export const useRpcData = (rpc: string) => {
	const memo = useMemo(() => {
		let result = { ANONYMOUS_RPC: '' };

		const url = new URL(rpc);
		if (url.origin === 'https://devrpc4.openblock.vip') {
			result = { ANONYMOUS_RPC: 'https://bfc-anonymous.openblock.vip/rpc' };
		} else if (url.origin === 'http://localhost:9000') {
			result = { ANONYMOUS_RPC: '' };
		} else if (url.origin === 'https://testrpc.benfen.org') {
			result = { ANONYMOUS_RPC: ' https://test-anonymous-rpc.openblock.vip/rpc' };
		}
		console.log('useRpcData::memo', result);
		return result;
	}, [rpc]);

	return memo;
};
