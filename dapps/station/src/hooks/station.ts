// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
/* eslint-disable @tanstack/query/exhaustive-deps */

import { useQuery } from '@tanstack/react-query';
import { useRpc } from '../context/RpcClientContext';
import { bcs, SUI_TYPE_ARG } from '@mysten/sui.js';

import { TransactionBlock } from '@mysten/sui.js/transactions';
import useDebounce from './useDebounce';
import { getAllCoins, normalizeSuiCoinType } from '~/utils/utils';
import BigNumber from 'bignumber.js';

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
			const swapAmount = new BigNumber(debouncedAmount).shiftedBy(9).toFixed()
			txb.moveCall({
				target: `0xc8::obc_system::${functionName}`,
				typeArguments: ['0xc8::usd::USD'],
				arguments: [txb.pure('0xc9'), txb.pure(swapAmount)],
			});

			const priceTxb = new TransactionBlock();
			priceTxb.moveCall({
				target: `0xc8::obc_system::${functionName}`,
				typeArguments: ['0xc8::usd::USD'],
				arguments: [priceTxb.pure('0xc9'), priceTxb.pure('1000000000')],
			});

			const resultExcute = provider.devInspectTransactionBlock({
				transactionBlock: txb,
				sender: currentAddress,
			});

			const priceResultExcute = provider.devInspectTransactionBlock({
				transactionBlock: priceTxb,
				sender: currentAddress,
			});


			const [result,priceResult] = await Promise.all([resultExcute,priceResultExcute])


			let decoded,priceDecoded,profitLoss;
			if (result?.results) {
				const returnValues = result.results[0].returnValues;
				if (returnValues) {
					const returnData = returnValues[0][0];
					decoded = returnData ? bcs.de('u64', Uint8Array.from(returnData)) : '';
				}
			}

			let gas = result.effects.gasUsed;
      		let total = BigInt(gas.computationCost) + BigInt(gas.storageCost) - BigInt(gas.storageRebate);

			if (priceResult?.results) {
				const returnValues = priceResult.results[0].returnValues;
				if (returnValues) {
					const returnData = returnValues[0][0];
					priceDecoded = returnData ? bcs.de('u64', Uint8Array.from(returnData)) : '';
				}
			}

			const expectAmount = new BigNumber(priceDecoded).multipliedBy(swapAmount).shiftedBy(-18).toFixed()
			const reciveAmount = decoded ? new BigNumber(decoded).shiftedBy(-9).toFixed() : ''

			if(expectAmount && reciveAmount && new BigNumber(expectAmount).isGreaterThan(reciveAmount)){
				profitLoss = new BigNumber(reciveAmount)
				.minus(expectAmount)
				.div(expectAmount)
				.multipliedBy(100)
				.toFixed(2);
			}
			console.log('priceDecodedpriceDecoded',reciveAmount, expectAmount,profitLoss)
			return {
				result: reciveAmount,
				expectAmount,
				profitLoss,
				gas: total ? new BigNumber( total.toString()).shiftedBy(-9).toFixed()  : ''
			};
		},
	});
}

export function useBalance(type: string, address: string) {
	const provider = useRpc();
	return useQuery({
		queryKey: ['get', 'balance', type, address],
		enabled: !!address,
		queryFn: async () => {
			const typeArg = type === 'mint' ? SUI_TYPE_ARG : '0xc8::usd::USD';

			const coinType = normalizeSuiCoinType(typeArg);

			console.log('coinTypecoinType',typeArg,coinType)

			const coins = await getAllCoins(provider, address, typeArg);

			return coins || [];
		},
		select: (coins) => {
			let result: any = 0;
			coins.forEach((item) => {
				result = result + Number(item.balance);
			});
			if (result > 0) {
				result = new BigNumber(result.toString()).shiftedBy(-9).toFixed();
			}
			console.log('resultresult', result);
			return result;
		},
	});
}
