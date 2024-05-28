// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

export function fromHEX(hexStr: string): Uint8Array {
<<<<<<< HEAD:sdk/typescript/src/bcs/src/hex.ts
	// @ts-ignore
	let intArr = (/^BFC/i.test(hexStr) ? hexStr.slice(3, -4) : hexStr.replace('0x', ''))
		.match(/.{1,2}/g)
		.map((byte) => parseInt(byte, 16));

	if (intArr === null) {
		throw new Error(`Unable to parse HEX: ${hexStr}`);
	}
=======
	const normalized = hexStr.startsWith('0x') ? hexStr.slice(2) : hexStr;
	const padded = normalized.length % 2 === 0 ? normalized : `0${normalized}}`;
	const intArr = padded.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) ?? [];
>>>>>>> mainnet-v1.24.1:sdk/bcs/src/hex.ts

	return Uint8Array.from(intArr);
}

export function toHEX(bytes: Uint8Array): string {
	return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');
}
