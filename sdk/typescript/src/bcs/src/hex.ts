// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

export function fromHEX(hexStr: string): Uint8Array {
	const normalized = /^BFC/i.test(hexStr) ? hexStr.slice(3, -4) : hexStr.replace(/^0x/, '');
	const padded = normalized.length % 2 === 0 ? normalized : `0${normalized}}`;
	const intArr = padded.match(/.{2}/g)?.map((byte) => parseInt(byte, 16)) ?? [];

	return Uint8Array.from(intArr);
}

export function toHEX(bytes: Uint8Array): string {
	return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');
}
