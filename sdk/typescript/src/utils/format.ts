// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { toHEX } from '../bcs/src/index.js';
import sha256 from 'fast-sha256';

const ELLIPSIS = '\u{2026}';

export function sui2BfcAddress(suiAddress: string): string {
	if (/^BFC/i.test(suiAddress)) {
		return suiAddress;
	}
	const hex = suiAddress.replace(/^0x/, '').padStart(64, '0').toLowerCase();
	const hash = toHEX(sha256(new TextEncoder().encode(hex)));
	return `BFC${hex}${hash.slice(0, 4)}`;
}

export function bfc2SuiAddress(bfcAddress: string): string {
	if (bfcAddress.startsWith('0x') || !/^BFC/i.test(bfcAddress)) {
		return bfcAddress;
	}
	return `0x${bfcAddress.slice(3, -4)}`;
}

export function formatAddress(address: string) {
	let text = address;
	if (!/^BFC/i.test(address)) {
		text = sui2BfcAddress(address);
	}

	return `${text.slice(0, 4)}${ELLIPSIS}${text.slice(-4)}`;
}

export function formatDigest(digest: string) {
	// Use 10 first characters
	return `${digest.slice(0, 10)}${ELLIPSIS}`;
}
