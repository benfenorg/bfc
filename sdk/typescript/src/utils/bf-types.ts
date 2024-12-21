// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { bytesToHex, hexToBytes } from '@noble/hashes/utils';
import BigNumber from 'bignumber.js';

import { fromB58, splitGenericParameters } from '../bcs/src/index.js';
import { bfc2HexAddress } from './format.js';

const TX_DIGEST_LENGTH = 32;

/** Returns whether the tx digest is valid based on the serialization format */
export function isValidTransactionDigest(value: string): value is string {
	try {
		const buffer = fromB58(value);
		return buffer.length === TX_DIGEST_LENGTH;
	} catch (e) {
		return false;
	}
}

export const BENFEN_ADDRESS_LENGTH = 32;
export function isValidBenfenAddress(value: string): value is string {
	let address = value;
	if (/^bfc/i.test(value)) {
		address = value.slice(3, -4);
	}
	return isHex(address) && getHexByteLength(address) === BENFEN_ADDRESS_LENGTH;
}

export function isValidBenfenObjectId(value: string): boolean {
	return isValidBenfenAddress(value);
}

type StructTag = {
	address: string;
	module: string;
	name: string;
	typeParams: (string | StructTag)[];
};

function parseTypeTag(type: string): string | StructTag {
	if (!type.includes('::')) return type;

	return parseStructTag(type);
}

export function parseStructTag(type: string): StructTag {
	const [address, module] = type.split('::');

	const rest = type.slice(address.length + module.length + 4);
	const name = rest.includes('<') ? rest.slice(0, rest.indexOf('<')) : rest;
	const typeParams = rest.includes('<')
		? splitGenericParameters(rest.slice(rest.indexOf('<') + 1, rest.lastIndexOf('>'))).map(
				(typeParam) => parseTypeTag(typeParam.trim()),
		  )
		: [];

	return {
		address: normalizeHexAddress(address),
		module,
		name,
		typeParams,
	};
}

export function normalizeStructTag(type: string | StructTag): string {
	const { address, module, name, typeParams } =
		typeof type === 'string' ? parseStructTag(type) : type;

	const formattedTypeParams =
		typeParams?.length > 0
			? `<${typeParams
					.map((typeParam) =>
						typeof typeParam === 'string' ? typeParam : normalizeStructTag(typeParam),
					)
					.join(',')}>`
			: '';

	return `${address}::${module}::${name}${formattedTypeParams}`;
}

/**
 * Perform the following operations:
 * 1. Make the address lower case
 * 2. Prepend `0x` if the string does not start with `0x`.
 * 3. Add more zeros if the length of the address(excluding `0x`) is less than `BENFEN_ADDRESS_LENGTH`
 *
 * WARNING: if the address value itself starts with `0x`, e.g., `0x0x`, the default behavior
 * is to treat the first `0x` not as part of the address. The default behavior can be overridden by
 * setting `forceAdd0x` to true
 *
 */
export function normalizeHexAddress(value: string, forceAdd0x: boolean = false): string {
	let address = value.toLowerCase();
	if (/^bfc/i.test(value)) {
		address = bfc2HexAddress(value);
	}
	if (!forceAdd0x && address.startsWith('0x')) {
		address = address.slice(2);
	}
	return `0x${address.padStart(BENFEN_ADDRESS_LENGTH * 2, '0')}`;
}

export function normalizeBenfenObjectId(value: string, forceAdd0x: boolean = false): string {
	return normalizeHexAddress(value, forceAdd0x);
}

function isHex(value: string): boolean {
	return /^(0x|0X)?[a-fA-F0-9]+$/.test(value) && value.length % 2 === 0;
}

function getHexByteLength(value: string): number {
	return /^(0x|0X)/.test(value) ? (value.length - 2) / 2 : value.length / 2;
}

export function humanReadableToBfcDigits(amount: number | string) {
	return BigInt(new BigNumber(amount).shiftedBy(9).integerValue().toString(10));
}

export function bfcDigitsToHumanReadable(amount: string | number) {
	return new BigNumber(amount).shiftedBy(-9).toString(10);
}

export function hexToString(hex: string) {
	return new TextDecoder().decode(hexToBytes((hex || '').replace(/^0x/, '')));
}

export function strToHex(str: string) {
	return `0x${bytesToHex(new TextEncoder().encode(str || ''))}`;
}
