// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import sha256 from 'fast-sha256';

import { toHEX } from '../bcs/src/index.js';

const ELLIPSIS = '\u{2026}';

export function hex2BfcAddress(hexAddress: string): string {
	if (/^BFC/i.test(hexAddress)) {
		return hexAddress;
	}
	const hex = hexAddress.replace(/^0x/, '').padStart(64, '0').toLowerCase();
	const hash = toHEX(sha256(new TextEncoder().encode(hex)));
	return `BFC${hex}${hash.slice(0, 4)}`;
}

export function bfc2HexAddress(bfcAddress: string): string {
	if (bfcAddress.startsWith('0x') || !/^BFC/i.test(bfcAddress)) {
		return bfcAddress;
	}
	return `0x${bfcAddress.slice(3, -4)}`;
}

export function formatAddress(address: string) {
	let text = address;
	if (!/^BFC/i.test(address)) {
		text = hex2BfcAddress(address);
	}

	return `${text.slice(0, 4)}${ELLIPSIS}${text.slice(-4)}`;
}

export function formatDigest(digest: string) {
	// Use 10 first characters
	return `${digest.slice(0, 10)}${ELLIPSIS}`;
}
