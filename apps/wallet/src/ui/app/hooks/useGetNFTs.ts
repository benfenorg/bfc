// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type BenfenObjectData } from '@benfen/bfc.js/client';
import { hasDisplayData, useGetOwnedObjects } from '@mysten/core';
import { useMemo } from 'react';

import { useBuyNLargeAsset } from '../components/buynlarge/useBuyNLargeAsset';
import { useHiddenAssets } from '../pages/home/hidden-assets/HiddenAssetsProvider';

type OwnedAssets = {
	visual: BenfenObjectData[];
	other: BenfenObjectData[];
	hidden: BenfenObjectData[];
};

export enum AssetFilterTypes {
	visual = 'visual',
	other = 'other',
}

export function useGetNFTs(address?: string | null) {
	const { asset, objectType } = useBuyNLargeAsset();
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
			MatchNone: objectType
				? [{ StructType: '0x2::coin::Coin' }, { StructType: objectType }]
				: [{ StructType: '0x2::coin::Coin' }],
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
				if (hasDisplayData(curr)) acc.visual.push(curr.data as BenfenObjectData);
				if (!hasDisplayData(curr)) acc.other.push(curr.data as BenfenObjectData);
				if (hiddenAssetIds.includes(curr.data?.objectId!))
					acc.hidden.push(curr.data as BenfenObjectData);
				return acc;
			}, ownedAssets);

		if (asset?.data) {
			groupedAssets?.visual.unshift(asset.data);
		}

		return groupedAssets;
	}, [hiddenAssetIds, data?.pages, asset]);

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
