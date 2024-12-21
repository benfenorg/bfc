// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { fromB64, toB64 } from '../../bcs/index.js';
import { PublicKey } from '../../cryptography/publickey.js';
import type { PublicKeyInitData } from '../../cryptography/publickey.js';
import { SIGNATURE_SCHEME_TO_FLAG } from '../../cryptography/signature-scheme.js';
import type { SerializedSignature } from '../../cryptography/signature.js';
import { extractClaimValue } from './jwt-utils.js';
import { parseZkLoginSignature } from './signature.js';
import { toPaddedBigEndianBytes } from './utils.js';

/**
 * A zkLogin public identifier
 */
export class ZkLoginPublicIdentifier extends PublicKey {
	#data: Uint8Array;

	/**
	 * Create a new ZkLoginPublicIdentifier object
	 * @param value zkLogin public identifier as buffer or base-64 encoded string
	 */
	constructor(value: PublicKeyInitData) {
		super();

		if (typeof value === 'string') {
			this.#data = fromB64(value);
		} else if (value instanceof Uint8Array) {
			this.#data = value;
		} else {
			this.#data = Uint8Array.from(value);
		}
	}

	/**
	 * Checks if two zkLogin public identifiers are equal
	 */
	override equals(publicKey: ZkLoginPublicIdentifier): boolean {
		return super.equals(publicKey);
	}

	/**
	 * Return the byte array representation of the zkLogin public identifier
	 */
	toRawBytes(): Uint8Array {
		return this.#data;
	}

	/**
	 * Return the Benfen address associated with this ZkLogin public identifier
	 */
	flag(): number {
		return SIGNATURE_SCHEME_TO_FLAG['ZkLogin'];
	}

	/**
	 * Verifies that the signature is valid for for the provided message
	 */
	async verify(_message: Uint8Array, _signature: Uint8Array | string): Promise<boolean> {
		throw Error('does not support');
	}

	/**
	 * Verifies that the signature is valid for for the provided PersonalMessage
	 */
	verifyPersonalMessage(
		message: Uint8Array,
		signature: Uint8Array | SerializedSignature,
	): Promise<boolean> {
		return Promise.resolve(true);
	}

	/**
	 * Verifies that the signature is valid for for the provided TransactionBlock
	 */
	verifyTransactionBlock(
		transactionBlock: Uint8Array,
		signature: Uint8Array | SerializedSignature,
	): Promise<boolean> {
		return Promise.resolve(true);
	}
}

// Derive the public identifier for zklogin based on address seed and iss.
export function toZkLoginPublicIdentifier(
	addressSeed: bigint,
	iss: string,
): ZkLoginPublicIdentifier {
	// Consists of iss_bytes_len || iss_bytes || padded_32_byte_address_seed.
	const addressSeedBytesBigEndian = toPaddedBigEndianBytes(addressSeed, 32);
	const issBytes = new TextEncoder().encode(iss);
	const tmp = new Uint8Array(1 + issBytes.length + addressSeedBytesBigEndian.length);
	tmp.set([issBytes.length], 0);
	tmp.set(issBytes, 1);
	tmp.set(addressSeedBytesBigEndian, 1 + issBytes.length);
	return new ZkLoginPublicIdentifier(tmp);
}

export function parseSerializedZkLoginSignature(signature: Uint8Array | SerializedSignature) {
	const bytes = typeof signature === 'string' ? fromB64(signature) : signature;

	if (bytes[0] !== SIGNATURE_SCHEME_TO_FLAG.ZkLogin) {
		throw new Error('Invalid signature scheme');
	}

	const signatureBytes = bytes.slice(1);
	const { inputs, maxEpoch, userSignature } = parseZkLoginSignature(signatureBytes);
	const { issBase64Details, addressSeed } = inputs;
	const iss = extractClaimValue<string>(issBase64Details, 'iss');
	const publicIdentifer = toZkLoginPublicIdentifier(BigInt(addressSeed), iss);
	const address = publicIdentifer.toHexAddress();
	return {
		serializedSignature: toB64(bytes),
		signatureScheme: 'ZkLogin' as const,
		zkLogin: {
			inputs,
			maxEpoch,
			userSignature,
			iss,
			address,
			addressSeed: BigInt(addressSeed),
		},
		signature: bytes,
		publicKey: publicIdentifer.toRawBytes(),
	};
}
