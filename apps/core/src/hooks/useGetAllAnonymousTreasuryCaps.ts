// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

import { ANONYMOUS_COIN_TREASURY_CAP } from '../utils/constants';

export type AnonymousTreasuryCapFields = {
	id: {
		id: string;
	};
	total_supply: {
		type: string;
		fields: {
			value: string;
		};
	};
	type: string;
};

export const useGetAllAnonymousTreasuryCaps = (address?: string | null) => {
	const client = useBenfenClient();

	return useQuery({
		queryKey: ['get-all-anonymous-treasury-caps', address],
		queryFn: async () => {
			const result: AnonymousTreasuryCapFields[] = [];
			let cursor: string | undefined | null = undefined;

			for (;;) {
				const data = await client.getOwnedObjects({
					owner: address!,
					filter: {
						MatchAll: [{ StructType: ANONYMOUS_COIN_TREASURY_CAP }],
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
								return {
									...content.fields,
									type: content.type,
								} as AnonymousTreasuryCapFields;
							}
							return undefined;
						})
						.filter((row): row is AnonymousTreasuryCapFields => !!row),
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
