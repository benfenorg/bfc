// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMemo } from 'react';

import type { SuiParsedData } from '@benfen/bfc.js/client';

export const parseIpfsUrl = (ipfsUrl: string) =>
	ipfsUrl.replace(/^ipfs:\/\//, 'https://ipfs.io/ipfs/');

export default function useMediaUrl(objData: SuiParsedData | null) {
	const { fields } =
		((objData?.dataType === 'moveObject' && objData) as {
			fields: { url?: string; metadata?: { fields: { url: string } } };
		}) || {};

	return useMemo(() => {
		if (fields) {
			const mediaUrl = fields.url || fields.metadata?.fields.url;
			if (typeof mediaUrl === 'string') {
				return parseIpfsUrl(mediaUrl);
			}
		}
		return null;
	}, [fields]);
}
