// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
<<<<<<< HEAD
import { DisplayFieldsResponse, SuiObjectResponse } from '@benfen/bfc.js/client';
=======
import { DisplayFieldsResponse, SuiObjectResponse } from '@mysten/sui.js/client';

>>>>>>> mainnet-v1.24.1
import { hasDisplayData } from '../hasDisplayData';

export function getObjectDisplayLookup(objects: SuiObjectResponse[] = []) {
	const lookup: Map<string, DisplayFieldsResponse> = new Map();
	return objects?.filter(hasDisplayData).reduce((acc, curr) => {
		if (curr.data?.objectId) {
			acc.set(curr.data.objectId, curr.data.display as DisplayFieldsResponse);
		}
		return acc;
	}, lookup);
}
