// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export const ANONYMOUS_COIN_TYPE = '0x2::anonymous_coin::Anonymous_Coin';

export type AnonymousCoinFields = {
	balance: {
		type: string;
		fields: {
			balance_type: {
				type: string;
				variant: string;
			};
			encode_data: string;
			value: string;
			value1: string;
			value2: string;
			version: number;
		};
	};
	id: {
		id: string;
	};
};

export const useGetAllAnonymousCoins = (address?: string | null) => {
	const client = useBenfenClient();

	return useQuery({
		queryKey: ['get-all-anonymous-coins', address],
		queryFn: async () => {
			const result: AnonymousCoinFields[] = [];
			let cursor: string | undefined | null = undefined;

			for (;;) {
				const data = await client.getOwnedObjects({
					owner: address!,
					filter: {
						MatchAll: [{ StructType: ANONYMOUS_COIN_TYPE }],
					},
					options: {
						showType: true,
						showContent: true,
						showDisplay: true,
					},
					limit: 50,
					cursor,
				});
				result.push(
					...(data.data ?? [])
						.map((row) => {
							const content = row.data?.content;
							if (content?.dataType === 'moveObject') {
								return content.fields as AnonymousCoinFields;
							}
							return undefined;
						})
						.filter((row): row is AnonymousCoinFields => !!row),
				);

				cursor = data.nextCursor;
				if (!data.hasNextPage) {
					break;
				}
			}

			return result;
		},
		refetchInterval: 10_000,
		staleTime: 5_000,
	});
};
