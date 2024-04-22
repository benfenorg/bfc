// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { getObjectFields } from '@benfen/bfc.js';
import { type Vote } from '@benfen/bfc.js/client';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetBFCDaoVote(address: string) {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['dao', 'vote', address],
		enabled: Boolean(address),
		queryFn: () =>
			client
				.getOwnedObjects({
					owner: address,
					filter: {
						StructType: '0xc8::bfc_dao::Vote',
					},
					options: {
						showType: true,
						showContent: true,
					},
				})
				.then((res: any) => {
					if (res?.data?.length > 0) {
						return res.data;
					}
					return [];
				}),
		select: (data): Vote[] => data.map((item: any) => getObjectFields(item!) as Vote),
	});
}
