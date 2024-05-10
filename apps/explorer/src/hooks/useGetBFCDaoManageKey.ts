// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetBFCDaoManageKey(address: string) {
	const client = useSuiClient();

	return useQuery({
		queryKey: ['dao', 'manageKey', address],
		enabled: Boolean(address),
		queryFn: () =>
			client
				.getOwnedObjects({
					owner: address,
					filter: {
						StructType:
							'0x00000000000000000000000000000000000000000000000000000000000000c8::bfc_dao_manager::BFCDaoManageKey',
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
