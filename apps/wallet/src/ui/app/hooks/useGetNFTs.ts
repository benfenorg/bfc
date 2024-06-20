// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiObjectData } from '@benfen/bfc.js/client';
import { hasDisplayData, useGetOwnedObjects } from '@mysten/core';

export function useGetNFTs(address?: string | null) {
	const {
		data,
		isLoading,
		error,
		isError,
		isFetchingNextPage,
		hasNextPage,
		fetchNextPage,
		isInitialLoading,
	} = useGetOwnedObjects(
		address,
		{
			MatchNone: [{ StructType: '0x2::coin::Coin' }],
		},
		50,
	);

	const ownedAssets =
		data?.pages
			.flatMap((page) => page.data)
			.sort((object) => (hasDisplayData(object) ? -1 : 1))
			.map(({ data }) => data as SuiObjectData) ?? [];

	return {
		data: ownedAssets,
		isInitialLoading,
		hasNextPage,
		isFetchingNextPage,
		fetchNextPage,
		isLoading: isLoading,
		isError: isError,
		error,
	};
}
