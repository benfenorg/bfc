// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
/* eslint-disable @tanstack/query/exhaustive-deps */

import { useQuery } from '@tanstack/react-query';
import { useRpc } from '../context/RpcClientContext';
import { bcs, SUI_TYPE_ARG } from '@mysten/sui.js';

import { TransactionBlock } from '@mysten/sui.js/transactions';
import useDebounce from './useDebounce';
import { getAllCoins, normalizeSuiCoinType } from '~/utils/utils';
import BigNumber from "bignumber.js";


export function useStationQuery(type: string, amount: string) {
	const provider = useRpc();
	const debouncedAmount = useDebounce(amount, 1000);

	const isEnabled = Number(debouncedAmount) && Number(debouncedAmount) > 0 ? true : false;
	return useQuery({
		queryKey: ['get', type, debouncedAmount],
		enabled: isEnabled,
		refetchInterval: 60000,
		staleTime: 1000,
		queryFn: async () => {
			const txb = new TransactionBlock();
			const currentAddress = '0x0000000000000000000000000000000000000000000000000000000000000000';
			const functionName = type === 'mint' ? 'get_stablecoin_by_obc' : 'get_obc_by_stablecoin';
			txb.moveCall({
				target: `0xc8::obc_system::${functionName}`,
				typeArguments: ['0xc8::usd::USD'],
				arguments: [txb.pure('0xc9'), txb.pure(debouncedAmount)],
			});
			const result = await provider.devInspectTransactionBlock({
				transactionBlock: txb,
				sender: currentAddress,
			});

			let decoded;
			if (result?.results) {
				const returnValues = result.results[0].returnValues;
				if (returnValues) {
					const returnData = returnValues[0][0];
					decoded = returnData ? bcs.de('u64', Uint8Array.from(returnData)) : '';
				}
			}
			console.log('decodeddecoded', decoded);
			return decoded;
		},
	});
}

export function useBalance(type: string, address: string) {
	const provider = useRpc();
	return useQuery({
		queryKey: ['get', 'balance', type],
		enabled: !!address,
		queryFn: async () => {
			const typeArg = type === 'mint' ? SUI_TYPE_ARG : '0xc8::usd::USD';

			const coinType = normalizeSuiCoinType(typeArg);

			const coinsData = await getAllCoins(provider, address, coinType);

			const coins = coinsData?.filter(({ lockedUntilEpoch: lock }) => !lock);

			return coins || []
		},
		select:(coins)=>{
			let result:any = 0;
			coins.forEach((item)=>{result = result + Number(item.balance)})
			if(result > 0){
				result = new BigNumber(result.toString()).shiftedBy(-9).toFixed();
			}
			console.log('resultresult',result)
			return result
		}
	});
}

