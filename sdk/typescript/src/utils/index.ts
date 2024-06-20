// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	bfcDigitsToHumanReadable,
	hexToString,
	humanReadableToBfcDigits,
	isValidSuiAddress,
	isValidSuiObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeSuiAddress,
	normalizeSuiObjectId,
	parseStructTag,
	strToHex,
	SUI_ADDRESS_LENGTH,
} from './bfc-types.js';
import { bfc2SuiAddress, formatAddress, formatDigest, sui2BfcAddress } from './format.js';

export { fromB64, toB64, fromHEX, toHEX } from '../bcs/src/index.js';
export { is, assert } from 'superstruct';

export {
	formatAddress,
	formatDigest,
	isValidSuiAddress,
	isValidSuiObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeSuiAddress,
	normalizeSuiObjectId,
	parseStructTag,
	SUI_ADDRESS_LENGTH,
	humanReadableToBfcDigits,
	bfcDigitsToHumanReadable,
	hexToString,
	strToHex,
	sui2BfcAddress,
	bfc2SuiAddress,
};

export { isValidSuiNSName, normalizeSuiNSName } from './suins.js';

export const SUI_DECIMALS = 9;
export const MIST_PER_SUI = BigInt(1000000000);

export const MOVE_STDLIB_ADDRESS = '0x1';
export const SUI_FRAMEWORK_ADDRESS = '0x2';
export const SUI_SYSTEM_ADDRESS = '0x3';
export const SUI_CLOCK_OBJECT_ID = normalizeSuiObjectId('0x6');
export const SUI_SYSTEM_MODULE_NAME = 'bfc_system';
export const SUI_TYPE_ARG = `${SUI_FRAMEWORK_ADDRESS}::bfc::BFC`;
export const SUI_SYSTEM_STATE_OBJECT_ID: string = sui2BfcAddress('0x5');
