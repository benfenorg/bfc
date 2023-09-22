// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useRpcClient } from '@mysten/core';
import { useQuery } from '@tanstack/react-query';

export function useGetOBCDaoManageKey(address: string) {
	const rpc = useRpcClient();

	return useQuery({
		queryKey: ['dao', 'manageKey', address],
		enabled: Boolean(address),
		queryFn: () =>
			rpc
				.getOwnedObjects({
					owner: address,
					filter: {
						StructType:
							'0x00000000000000000000000000000000000000000000000000000000000000c8::obc_dao_manager::OBCDaoManageKey',
					},
					options: {
						showType: true,
					},
				})
				.then((res: any): string => {
					if (res?.data?.length > 0) {
						return res.data[0]?.data?.objectId ?? '';
					}
					return '';
				}),
	});
}
