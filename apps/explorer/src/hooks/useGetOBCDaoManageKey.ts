// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useRpcClient } from '@mysten/core';
import { useWalletKit } from '@mysten/wallet-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetOBCDaoManageKey() {
	const rpc = useRpcClient();
	const { currentAccount } = useWalletKit();
	return useQuery({
		queryKey: ['dao', 'object', currentAccount],
		enabled: Boolean(currentAccount?.address),
		queryFn: () =>
			rpc
				.getOwnedObjects({
					owner: currentAccount!.address,
					filter: {
						StructType:
							'0x00000000000000000000000000000000000000000000000000000000000000c8::obc_dao_manager::OBCDaoManageKey',
					},
					options: {
						showType: true,
					},
				})
				.then((res: any) => {
					if (res?.data?.length > 0) {
						return res.data[0]?.data?.objectId ?? '';
					}
					return '';
				}),
	});
}
