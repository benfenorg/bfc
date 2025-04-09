// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

export function getFullnodeUrl(network: 'mainnet' | 'testnet' | 'devnet' | 'localnet') {
	switch (network) {
		case 'mainnet':
			return 'https://rpc-mainnet.benfen.org';
		case 'testnet':
			return 'https://testrpc.benfen.org';
		case 'devnet':
			return 'https://devrpc.benfen.org';
		case 'localnet':
			return 'http://127.0.0.1:9000';
		default:
			throw new Error(`Unknown network: ${network}`);
	}
}
