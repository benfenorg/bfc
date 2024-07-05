// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type BenfenClient } from '@benfen/bfc.js/client';
import { useBenfenClient } from '@benfen/bfc.js/dapp-kit';
import { isValidBenfenAddress } from '@benfen/bfc.js/utils';
import { isSuiNSName, useSuiNSEnabled } from '@mysten/core';
import { useMemo } from 'react';
import * as Yup from 'yup';

export function createSuiAddressValidation(client: BenfenClient, suiNSEnabled: boolean) {
	const resolveCache = new Map<string, boolean>();

	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-sui-address', 'Invalid address. Please check again.', async (value) => {
			if (suiNSEnabled && isSuiNSName(value)) {
				if (resolveCache.has(value)) {
					return resolveCache.get(value)!;
				}

				const address = await client.resolveNameServiceAddress({
					name: value,
				});

				resolveCache.set(value, !!address);

				return !!address;
			}

			return isValidBenfenAddress(value);
		})
		.label("Recipient's address");
}

export function useSuiAddressValidation() {
	const client = useBenfenClient();
	const suiNSEnabled = useSuiNSEnabled();

	return useMemo(() => {
		return createSuiAddressValidation(client, suiNSEnabled);
	}, [client, suiNSEnabled]);
}
