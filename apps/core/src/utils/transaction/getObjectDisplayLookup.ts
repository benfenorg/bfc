// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { DisplayFieldsResponse, BenfenObjectResponse } from '@benfen/bfc.js/client';

import { hasDisplayData } from '../hasDisplayData';

export function getObjectDisplayLookup(objects: BenfenObjectResponse[] = []) {
	const lookup: Map<string, DisplayFieldsResponse> = new Map();
	return objects?.filter(hasDisplayData).reduce((acc, curr) => {
		if (curr.data?.objectId) {
			acc.set(curr.data.objectId, curr.data.display as DisplayFieldsResponse);
		}
		return acc;
	}, lookup);
}
