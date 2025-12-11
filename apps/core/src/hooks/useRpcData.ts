// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

export const useRpcData = (rpc: string) => {
	const memo = useMemo(() => {
		let ANONYMOUS_RPC = '';

		const url = new URL(rpc);
		if (url.origin === 'https://devrpc4.openblock.vip') {
			ANONYMOUS_RPC = 'https://bfc-anonymous.openblock.vip/rpc';
		} else if (url.origin === 'http://localhost:9000') {
			ANONYMOUS_RPC = '';
		} else if (url.origin === 'https://testrpc.benfen.org') {
			ANONYMOUS_RPC = 'https://test-anonymous-rpc.openblock.vip/rpc';
		} else if (url.origin === 'https://stagerpc.benfen.org') {
			ANONYMOUS_RPC = 'https://stagerpc.benfen.org/rpc';
		}
		const result = { ANONYMOUS_RPC };
		console.log('useRpcData::memo', result);
		return result;
	}, [rpc]);

	return memo;
};
