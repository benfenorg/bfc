// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { toHEX } from '@mysten/bcs';
import sha256 from 'fast-sha256';

const ELLIPSIS = '\u{2026}';

export function sui2ObcAddress(suiAddress: string): string {
	if (suiAddress.startsWith('OBC')) {
		return suiAddress;
	}
	const hex = suiAddress.replace(/^0x/, '').padStart(64, '0').toLowerCase();
	const hash = toHEX(sha256(new TextEncoder().encode(hex)));
	return `OBC${hex}${hash.slice(0, 4)}`;
}

export function obc2SuiAddress(obcAddress: string): string {
	if (obcAddress.startsWith('0x') || !/^OBC/i.test(obcAddress)) {
		return obcAddress;
	}
	return `0x${obcAddress.slice(3, -4)}`;
}

export function formatAddress(address: string) {
	let text = address;
	if (!address.startsWith('OBC')) {
		text = sui2ObcAddress(address);
	}

	return `${text.slice(0, 4)}${ELLIPSIS}${text.slice(-4)}`;
}

export function formatDigest(digest: string) {
	// Use 10 first characters
	return `${digest.slice(0, 10)}${ELLIPSIS}`;
}
