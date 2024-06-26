// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useGetObject } from '@mysten/core';
import { useMemo } from 'react';

export function useOwnedNFT(nftObjectId: string | null, address: string | null) {
	const data = useGetObject(nftObjectId);
	const { data: objectData, isPending } = data;

	const objectDetails = useMemo(() => {
		if (!objectData || !objectData.data || !address) return null;
		const objectOwner = objectData.data.owner;
		const data =
			objectOwner &&
			objectOwner !== 'Immutable' &&
			'AddressOwner' in objectOwner &&
			objectOwner.AddressOwner === address
				? objectData.data
				: null;
		return data;
	}, [address, objectData]);

	return { ...data, isPending: isPending, data: objectDetails };
}
