// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useRpcClient } from '@mysten/core';
import { getObjectFields } from '@mysten/sui.js';
import { useQuery } from '@tanstack/react-query';

export function useGetOBCDaoVotingObc(address: string) {
	const rpc = useRpcClient();
	return useQuery({
		queryKey: ['dao', 'votingobc', address],
		enabled: Boolean(address),
		queryFn: () =>
			rpc
				.getOwnedObjects({
					owner: address,
					filter: {
						StructType:
							'0x00000000000000000000000000000000000000000000000000000000000000c8::voting_pool::VotingObc',
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
		select: (data) => data.map((item: any) => getObjectFields(item!)),
	});
}
