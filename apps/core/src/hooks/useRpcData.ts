// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

export const useRpcData = (rpc: string) => {
	const memo = useMemo(() => {
		let result = {};

		const url = new URL(rpc);
		if (url.origin === 'https://devrpc4.openblock.vip') {
			result = {};
		} else if (url.origin === 'http://localhost:9000') {
			result = {};
		}
		console.log('useRpcData::memo', result);
		return result;
	}, [rpc]);

	return memo;
};
