// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useContext } from 'react';

import type { BenfenClient } from '../../client/index.js';
import { BenfenClientContext } from '../components/BenfenClientProvider.js';

export function useBenfenClientContext() {
	const benfenClient = useContext(BenfenClientContext);

	if (!benfenClient) {
		throw new Error(
			'Could not find BenfenClientContext. Ensure that you have set up the BenfenClientProvider',
		);
	}

	return benfenClient;
}

export function useBenfenClient(): BenfenClient {
	return useBenfenClientContext().client;
}
