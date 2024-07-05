// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { fromB64 } from '../bcs/src/index.js';
import type { SignatureFlag } from '../cryptography/index.js';
import { SIGNATURE_FLAG_TO_SCHEME } from '../cryptography/index.js';
import { publicKeyFromRawBytes } from '../verify/index.js';

export * from './publickey.js';

export function publicKeyFromBenfenBytes(publicKey: string | Uint8Array) {
	const bytes = typeof publicKey === 'string' ? fromB64(publicKey) : publicKey;

	const signatureScheme = SIGNATURE_FLAG_TO_SCHEME[bytes[0] as SignatureFlag];

	if (signatureScheme === 'ZkLogin') {
		throw new Error('ZkLogin publicKey is not supported');
	}

	return publicKeyFromRawBytes(signatureScheme, bytes.slice(1));
}
