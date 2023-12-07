// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { getObjectFields } from '@benfen/bfc.js';
import { type VotingBfc } from '@benfen/bfc.js/client';
import { useSuiClient } from '@mysten/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetBFCDaoVotingBfc(address: string) {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['dao', 'votingBfc', address],
		enabled: Boolean(address),
		queryFn: () =>
			client
				.getOwnedObjects({
					owner: address,
					filter: {
						StructType: '0xc8::voting_pool::VotingBfc',
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
		select: (data): VotingBfc[] => data.map((item: any) => getObjectFields(item!) as VotingBfc),
	});
}
