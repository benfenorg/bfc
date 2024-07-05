// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import {
	BENFEN_ADDRESS_LENGTH,
	bfcDigitsToHumanReadable,
	hexToString,
	humanReadableToBfcDigits,
	isValidBenfenAddress,
	isValidBenfenObjectId,
	isValidTransactionDigest,
	normalizeBenfenObjectId,
	normalizeHexAddress,
	normalizeStructTag,
	parseStructTag,
	strToHex,
} from './bf-types.js';
import { bfc2HexAddress, formatAddress, formatDigest, hex2BfcAddress } from './format.js';

export { fromB64, toB64, fromHEX, toHEX } from '../bcs/src/index.js';
export { is, assert } from 'superstruct';

export {
	formatAddress,
	formatDigest,
	isValidBenfenAddress,
	isValidBenfenObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeHexAddress,
	normalizeBenfenObjectId,
	parseStructTag,
	BENFEN_ADDRESS_LENGTH,
	humanReadableToBfcDigits,
	bfcDigitsToHumanReadable,
	hexToString,
	strToHex,
	hex2BfcAddress,
	bfc2HexAddress,
};

export { isValidBenfenNSName, normalizeBenfenNSName } from './benfenns.js';

export const BFC_DECIMALS = 9;
export const MIST_PER_BFC = BigInt(1000000000);

export const MOVE_STDLIB_ADDRESS = '0x1';
export const BENFEN_FRAMEWORK_ADDRESS = '0x2';
export const BENFEN_SYSTEM_ADDRESS = '0x3';
export const BENFEN_CLOCK_OBJECT_ID = normalizeBenfenObjectId('0x6');
export const BFC_SYSTEM_MODULE_NAME = 'bfc_system';
export const BFC_TYPE_ARG = `${BENFEN_FRAMEWORK_ADDRESS}::bfc::BFC`;
export const BFC_SYSTEM_STATE_OBJECT_ID: string = hex2BfcAddress('0x5');
