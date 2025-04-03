// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { normalizeHexAddress } from '@benfen/bfc.js/utils';
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
	const client = useBenfenClient();
	const normalizedObjId = objectId && normalizeHexAddress(objectId);
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
