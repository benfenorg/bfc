// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

export const useRpcData = (rpc: string) => {
	const memo = useMemo(() => {
		let result = {
			ANONYMOUS_SWAP_POOL: '',
			ANONYMOUS_STABLE_PKG: '',
		};

		const url = new URL(rpc);
		if (url.origin === 'https://devrpc4.openblock.vip') {
			result = {
				ANONYMOUS_SWAP_POOL:
					'BFC116765bb51f7c24d4e75f326beef7439fbbdac7e8997020dea799f7986675cf3f13e',
				ANONYMOUS_STABLE_PKG:
					'BFC365cbc0fc12fb3928353a3b8f18a9b8a0d2bd5ecf5c20b08b4f30726ec5aaa626fa3',
			};
		} else if (url.origin === 'http://localhost:9000') {
			result = {
				ANONYMOUS_SWAP_POOL:
					'BFC59671ab230e41bd0e9d8a21dc431e2989490397bc5dadbced0b334807606d6aa36cf',
				ANONYMOUS_STABLE_PKG: '',
			};
		}
		console.log('useRpcData::memo', result);
		return result;
	}, [rpc]);

	return memo;
};
