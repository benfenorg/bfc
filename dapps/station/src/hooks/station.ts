// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
/* eslint-disable @tanstack/query/exhaustive-deps */

import { useQuery } from '@tanstack/react-query';
import {
	TANSTACK_KIOSK_DATA_KEY,
	TANSTACK_KIOSK_KEY,
	TANSTACK_OWNED_KIOSK_KEY,
} from '../utils/constants';
import { useRpc } from '../context/RpcClientContext';
import { bcs, ObjectId, SuiAddress, SuiObjectResponse, SUI_TYPE_ARG } from '@mysten/sui.js';
import {
	Kiosk,
	KioskData,
	KioskItem,
	KioskListing,
	KioskOwnerCap,
	fetchKiosk,
	getKioskObject,
	getOwnedKiosks,
} from '@mysten/kiosk';
import { parseObjectDisplays, processKioskListings } from '../utils/utils';
import { OwnedObjectType } from '../components/Inventory/OwnedObjects';
import { TransactionBlock } from '@mysten/sui.js/transactions';
import useDebounce from './useDebounce';
import { getAllCoins, normalizeSuiCoinType } from '~/utils/utils';
import BigNumber from "bignumber.js";

export type KioskFnType = (item: OwnedObjectType, price?: string) => Promise<void> | void;

/**
 * A helper to get user's kiosks.
 * If the user doesn't have a kiosk, the return is an object with null values.
 */
export function useOwnedKiosk(address: SuiAddress | undefined) {
	const provider = useRpc();

	return useQuery({
		queryKey: [TANSTACK_OWNED_KIOSK_KEY, address],
		refetchOnMount: false,
		retry: false,
		queryFn: async (): Promise<{
			caps: KioskOwnerCap[];
			kioskId: SuiAddress | undefined;
			kioskCap: SuiAddress | undefined;
		} | null> => {
			if (!address) return null;

			const { kioskOwnerCaps, kioskIds } = await getOwnedKiosks(provider, address);

			return {
				caps: kioskOwnerCaps,
				kioskId: kioskIds[0],
				kioskCap: kioskOwnerCaps[0]?.objectId,
			};
		},
	});
}

/**
 * A hook to fetch a kiosk (items, listings, etc) by its id.
 */
export function useKiosk(kioskId: string | undefined | null) {
	const provider = useRpc();

	return useQuery({
		queryKey: [TANSTACK_KIOSK_KEY, kioskId],
		queryFn: async (): Promise<{
			kioskData: KioskData | null;
			items: SuiObjectResponse[];
		}> => {
			if (!kioskId) return { kioskData: null, items: [] };
			const { data: res } = await fetchKiosk(
				provider,
				kioskId,
				{ limit: 1000 },
				{
					withKioskFields: true,
					withListingPrices: true,
				},
			);

			// get the items from rpc.
			const items = await provider.multiGetObjects({
				ids: res.itemIds,
				options: { showDisplay: true, showType: true },
			});

			return {
				kioskData: res,
				items,
			};
		},
		retry: false,
		select: ({
			items,
			kioskData,
		}): {
			items: OwnedObjectType[];
			listings: Record<ObjectId, KioskListing>;
		} => {
			if (!kioskData) return { items: [], listings: {} };
			// parse the displays for FE.
			const displays = parseObjectDisplays(items) || {};

			// attach the displays to the objects.
			const ownedItems = kioskData.items.map((item: KioskItem) => {
				return {
					...item,
					display: displays[item.objectId] || {},
				};
			});

			// return the items & listings.
			return {
				items: ownedItems,
				listings: processKioskListings(kioskData.items.map((x) => x.listing) as KioskListing[]),
			};
		},
	});
}

/**
 * A hook to fetch a kiosk's details.
 */
export function useKioskDetails(kioskId: string | undefined | null) {
	const provider = useRpc();

	return useQuery({
		queryKey: [TANSTACK_KIOSK_DATA_KEY, kioskId],
		queryFn: async (): Promise<Kiosk | null> => {
			if (!kioskId) return null;
			return await getKioskObject(provider, kioskId);
		},
	});
}

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

