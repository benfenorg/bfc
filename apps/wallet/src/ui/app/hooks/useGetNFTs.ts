// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiObjectData } from '@benfen/bfc.js/client';
import { hasDisplayData, isKioskOwnerToken, useGetOwnedObjects } from '@mysten/core';
<<<<<<< HEAD
=======
import { useKioskClient } from '@mysten/core/src/hooks/useKioskClient';
import { type SuiObjectData } from '@mysten/sui.js/client';
import { useMemo } from 'react';

import { useBuyNLargeAsset } from '../components/buynlarge/useBuyNLargeAsset';
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
>>>>>>> mainnet-v1.24.1

export function useGetNFTs(address?: string | null) {
	const kioskClient = useKioskClient();
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

<<<<<<< HEAD
	const ownedAssets =
		data?.pages
			.flatMap((page) => page.data)
			.sort((object) => (hasDisplayData(object) ? -1 : 1))
			.sort((object) => (isKioskOwnerToken(object) ? -1 : 1))
			.map(({ data }) => data as SuiObjectData) ?? [];

	return {
		data: ownedAssets,
		isInitialLoading,
=======
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
				if (hasDisplayData(curr) || isKioskOwnerToken(kioskClient.network, curr))
					acc.visual.push(curr.data as SuiObjectData);
				if (!hasDisplayData(curr)) acc.other.push(curr.data as SuiObjectData);
				if (hiddenAssetIds.includes(curr.data?.objectId!))
					acc.hidden.push(curr.data as SuiObjectData);
				return acc;
			}, ownedAssets);

		if (asset?.data) {
			groupedAssets?.visual.unshift(asset.data);
		}

		return groupedAssets;
	}, [hiddenAssetIds, data?.pages, kioskClient.network, asset]);

	return {
		data: assets,
		isLoading,
>>>>>>> mainnet-v1.24.1
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isPending: isPending,
		isError: isError,
		error,
	};
}
