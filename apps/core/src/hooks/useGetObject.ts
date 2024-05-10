// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { sui2BfcAddress } from '@benfen/bfc.js';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { useQuery } from '@tanstack/react-query';

const defaultOptions = {
	showType: true,
	showContent: true,
	showOwner: true,
	showPreviousTransaction: true,
	showStorageRebate: true,
	showDisplay: true,
};

export function useGetObject(objectId?: string | null) {
	const client = useSuiClient();
	const normalizedObjId = objectId && sui2BfcAddress(objectId);
	return useQuery({
		queryKey: ['object', normalizedObjId],
		queryFn: () =>
			client.getObject({
				id: normalizedObjId!,
				options: defaultOptions,
			}),
		enabled: !!normalizedObjId,
	});
}
