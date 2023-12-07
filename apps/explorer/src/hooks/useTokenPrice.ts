// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useSuiClient } from '@mysten/dapp-kit';
import { useQuery } from '@tanstack/react-query';
import { TransactionBlock } from '@benfen/bfc.js/transactions';
import { bcs } from '@benfen/bfc.js/bcs';
import BigNumber from 'bignumber.js';

const VaultInfo = {
	vault_id: 'address',
	position_number: 'u32',
	state: 'u8',
	state_counter: 'u32',
	max_counter_times: 'u32',
	last_sqrt_price: 'u128',
	coin_a_balance: 'u64',
	coin_b_balance: 'u64',
	coin_a_type: 'string',
	coin_b_type: 'string',
	tick_spacing: 'u32',
	spacing_times: 'u32',
	liquidity: 'u128',
	current_sqrt_price: 'u128',
	current_tick_index: 'u32',
	is_pause: 'bool',
	index: 'u64',
	base_point: 'u64',
};

export function useTokenPrice() {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['token-price', 'home', 'overviews'],
		refetchInterval: 60000,
		staleTime: 1000,
		queryFn: async () => {
			const txb = new TransactionBlock();
			txb.moveCall({
				target: `0xc8::bfc_system::vault_info`,
				typeArguments: ['0xc8::busd::BUSD'],
				arguments: [txb.pure('0xc9')],
			});
			const result = await client.devInspectTransactionBlock({
				sender: '0x0000000000000000000000000000000000000000000000000000000000000000',
				transactionBlock: txb,
			});

			let decoded;

			if (result?.results) {
				const returnValues = result.results[0].returnValues;
				if (returnValues) {
					const returnData = returnValues[0][0];
					decoded = returnData ? bcs.de(VaultInfo, Uint8Array.from(returnData)) : '';
				}
			}
			const price = decoded?.current_sqrt_price
				? new BigNumber(decoded.current_sqrt_price)
						.div(2 ** 64)
						.pow(2)
						.pow(-1)
						.dp(6)
						.toFixed()
				: '';
			return price;
		},
	});
}
