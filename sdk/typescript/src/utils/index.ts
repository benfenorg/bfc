// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

export { formatAddress, formatDigest, hex2BfcAddress, bfc2HexAddress } from './format.js';
export {
	isValidBenfenAddress,
	isValidBenfenObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeHexAddress,
	normalizeBenfenObjectId,
	parseStructTag,
	BENFEN_ADDRESS_LENGTH,
} from './bf-types.js';

export {
	fromB64,
	toB64,
	fromHEX,
	toHex,
	toHEX,
	fromHex,
	fromBase64,
	toBase64,
	fromBase58,
	toBase58,
} from '../bcs/index.js';
export { isValidBenfenNSName, normalizeBenfenNSName } from './benfenns.js';

export {
	BFC_DECIMALS,
	MIST_PER_BFC,
	MOVE_STDLIB_ADDRESS,
	BENFEN_FRAMEWORK_ADDRESS,
	BENFEN_SYSTEM_ADDRESS,
	BENFEN_CLOCK_OBJECT_ID,
	BFC_SYSTEM_MODULE_NAME,
	BFC_TYPE_ARG,
	BFC_SYSTEM_STATE_OBJECT_ID,
} from './constants.js';

export { isValidNamedPackage, isValidNamedType } from './move-registry.js';

export { deriveDynamicFieldID } from './dynamic-fields.js';
