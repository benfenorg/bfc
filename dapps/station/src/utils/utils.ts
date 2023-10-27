// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiClient } from '@mysten/sui.js/client';
import { SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import {
	MIST_PER_SUI,
	ObjectId,
	SuiObjectResponse,
	getObjectDisplay,
	getObjectId,
    CoinStruct,
    PaginatedCoins,
} from '@mysten/sui.js';

import { normalizeSuiAddress } from '@mysten/sui.js/utils';

// Parse the display of a list of objects into a simple {object_id: display} map
// to use throughout the app.
export const parseObjectDisplays = (
	data: SuiObjectResponse[],
): Record<ObjectId, Record<string, string> | undefined> => {
	return data.reduce<Record<ObjectId, Record<string, string> | undefined>>(
		(acc, item: SuiObjectResponse) => {
			const display = getObjectDisplay(item)?.data;
			const id = getObjectId(item);
			acc[id] = display || undefined;
			return acc;
		},
		{},
	);
};


export const mistToSui = (mist: bigint | string | undefined) => {
	if (!mist) return 0;
	return Number(mist || 0) / Number(MIST_PER_SUI);
};

export const formatSui = (amount: number) => {
	return new Intl.NumberFormat('en-US', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 5,
	}).format(amount);
};

const MAX_COINS_PER_REQUEST = 50;

export async function getAllCoins(
    client: SuiClient,
    address: string,
    coinType: string | null,
  ): Promise<CoinStruct[]> {
    let cursor: string | null = null;
    const allData: CoinStruct[] = [];
    do {
      const { data, nextCursor }: PaginatedCoins = await client.getCoins({
        owner: address,
        coinType,
        cursor,
        limit: MAX_COINS_PER_REQUEST,
      });
      if (!data || !data.length) {
        break;
      }
  
      for (const item of data) {
        const normalCoinType = normalizeSuiCoinType(item.coinType);
        allData.push({
          ...item,
          coinType: normalCoinType,
        });
      }
  
      cursor = nextCursor;
    } while (cursor);
  
    return allData;
  }

  
export function normalizeSuiCoinType(coinType: string): string {
    if (coinType !== SUI_TYPE_ARG) {
      const [normalAddress, module, name] = coinType.split('::');
      if (module && name) {
        try {
          return `${normalizeSuiAddress(
            normalAddress,
          ).toLowerCase()}::${module}::${name}`;
        } catch {
          // pass
        }
      }
    }
    return coinType;
  }
  