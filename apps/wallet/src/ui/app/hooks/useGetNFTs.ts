// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type SuiObjectData } from '@benfen/bfc.js/client';
import { ANONYMOUS_COIN_TYPE, hasDisplayData, useGetOwnedObjects } from '@mysten/core';
import { useMemo } from 'react';

import { useHiddenAssets } from '../pages/home/hidden-assets/HiddenAssetsProvider';

type OwnedAssets = {
	visual: SuiObjectData[];
	other: SuiObjectData[];
	hidden: SuiObjectData[];
};

export enum AssetFilterTypes {
	visual = 'visual',
	other = 'other',
}

export function useGetNFTs(address?: string | null) {
	const {
		data,
		isPending,
		error,
		isError,
		isFetchingNextPage,
		hasNextPage,
		fetchNextPage,
		isLoading,
	} = useGetOwnedObjects(
		address,
		{
			MatchNone: [{ StructType: '0x2::coin::Coin' }, { StructType: ANONYMOUS_COIN_TYPE }],
		},
		50,
	);
	const { hiddenAssetIds } = useHiddenAssets();

	const assets = useMemo(() => {
		const ownedAssets: OwnedAssets = {
			visual: [],
			other: [],
			hidden: [],
		};

		const groupedAssets = data?.pages
			.flatMap((page) => page.data)
			.filter((asset) => !hiddenAssetIds.includes(asset.data?.objectId!))
			.reduce((acc, curr) => {
				if (hasDisplayData(curr)) acc.visual.push(curr.data as SuiObjectData);
				if (!hasDisplayData(curr)) acc.other.push(curr.data as SuiObjectData);
				if (hiddenAssetIds.includes(curr.data?.objectId!))
					acc.hidden.push(curr.data as SuiObjectData);
				return acc;
			}, ownedAssets);

		return groupedAssets;
	}, [hiddenAssetIds, data?.pages]);

	return {
		data: assets,
		isLoading,
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isPending: isPending,
		isError: isError,
		error,
	};
}
