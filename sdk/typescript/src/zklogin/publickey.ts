// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';

import { fromBase64, toBase64, toHex } from '../bcs/index.js';
import { PublicKey } from '../cryptography/publickey.js';
import type { PublicKeyInitData } from '../cryptography/publickey.js';
import { SIGNATURE_SCHEME_TO_FLAG } from '../cryptography/signature-scheme.js';
import { BENFEN_ADDRESS_LENGTH, normalizeHexAddress } from '../utils/bf-types.js';
import type { ZkLoginSignatureInputs } from './bcs.js';
import { extractClaimValue } from './jwt-utils.js';
import { parseZkLoginSignature } from './signature.js';
import { normalizeZkLoginIssuer, toBigEndianBytes, toPaddedBigEndianBytes } from './utils.js';

/**
 * A zkLogin public identifier
 */
export class ZkLoginPublicIdentifier extends PublicKey {
	#data: Uint8Array;
	#legacyAddress: boolean;

	/**
	 * Create a new ZkLoginPublicIdentifier object
	 * @param value zkLogin public identifier as buffer or base-64 encoded string
	 */
	constructor(value: PublicKeyInitData) {
		super();

		if (typeof value === 'string') {
			this.#data = fromBase64(value);
		} else if (value instanceof Uint8Array) {
			this.#data = value;
		} else {
			this.#data = Uint8Array.from(value);
		}
		this.#legacyAddress = this.#data.length !== this.#data[0] + 1 + 32;

		if (this.#legacyAddress) {
			this.#data = normalizeZkLoginPublicKeyBytes(this.#data);
		}
	}

	static fromProof(address: string, proof: ZkLoginSignatureInputs) {
		const { issBase64Details, addressSeed } = proof;
		const iss = extractClaimValue<string>(issBase64Details, 'iss');

		const legacyPublicKey = toZkLoginPublicIdentifier(BigInt(addressSeed), iss, {
			legacyAddress: true,
		});

		if (legacyPublicKey.toHexAddress() === address) {
			return legacyPublicKey;
		}

		const publicKey = toZkLoginPublicIdentifier(BigInt(addressSeed), iss, {
			legacyAddress: false,
		});

		if (publicKey.toHexAddress() !== address) {
			throw new Error('Proof does not match address');
		}

		return publicKey;
	}

	/**
	 * Checks if two zkLogin public identifiers are equal
	 */
	override equals(publicKey: ZkLoginPublicIdentifier): boolean {
		return super.equals(publicKey);
	}

	override toHexAddress(): string {
		if (this.#legacyAddress) {
			return this.#toLegacyAddress();
		}

		return super.toHexAddress();
	}

	#toLegacyAddress() {
		const legacyBytes = normalizeZkLoginPublicKeyBytes(this.#data, true);
		const addressBytes = new Uint8Array(legacyBytes.length + 1);
		addressBytes[0] = this.flag();
		addressBytes.set(legacyBytes, 1);
		return normalizeHexAddress(
			bytesToHex(blake2b(addressBytes, { dkLen: 32 })).slice(0, BENFEN_ADDRESS_LENGTH * 2),
		);
	}

	/**
	 * Return the byte array representation of the zkLogin public identifier
	 */
	toRawBytes(): Uint8Array {
		return this.#data;
	}

	/**
	 * Return the hex address associated with this ZkLogin public identifier
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
	verifyPersonalMessage(message: Uint8Array, signature: Uint8Array | string): Promise<boolean> {
		return Promise.resolve(true);
	}

	/**
	 * Verifies that the signature is valid for for the provided Transaction
	 */
	verifyTransaction(transaction: Uint8Array, signature: Uint8Array | string): Promise<boolean> {
		return Promise.resolve(true);
	}

	/**
	 * Verifies that the public key is associated with the provided address
	 */
	override verifyAddress(address: string): boolean {
		return address === super.toHexAddress() || address === this.#toLegacyAddress();
	}
}

// Derive the public identifier for zklogin based on address seed and iss.
export function toZkLoginPublicIdentifier(
	addressSeed: bigint,
	iss: string,
	options?: { legacyAddress?: boolean },
): ZkLoginPublicIdentifier {
	// Consists of iss_bytes_len || iss_bytes || padded_32_byte_address_seed.
	const addressSeedBytesBigEndian = options?.legacyAddress
		? toBigEndianBytes(addressSeed, 32)
		: toPaddedBigEndianBytes(addressSeed, 32);

	const issBytes = new TextEncoder().encode(normalizeZkLoginIssuer(iss));
	const tmp = new Uint8Array(1 + issBytes.length + addressSeedBytesBigEndian.length);
	tmp.set([issBytes.length], 0);
	tmp.set(issBytes, 1);
	tmp.set(addressSeedBytesBigEndian, 1 + issBytes.length);
	return new ZkLoginPublicIdentifier(tmp);
}

function normalizeZkLoginPublicKeyBytes(bytes: Uint8Array, legacyAddress = false) {
	const issByteLength = bytes[0] + 1;
	const addressSeed = BigInt(`0x${toHex(bytes.slice(issByteLength))}`);
	const seedBytes = legacyAddress
		? toBigEndianBytes(addressSeed, 32)
		: toPaddedBigEndianBytes(addressSeed, 32);
	const data = new Uint8Array(issByteLength + seedBytes.length);
	data.set(bytes.slice(0, issByteLength), 0);
	data.set(seedBytes, issByteLength);
	return data;
}

export function parseSerializedZkLoginSignature(signature: Uint8Array | string) {
	const bytes = typeof signature === 'string' ? fromBase64(signature) : signature;

	if (bytes[0] !== SIGNATURE_SCHEME_TO_FLAG.ZkLogin) {
		throw new Error('Invalid signature scheme');
	}

	const signatureBytes = bytes.slice(1);
	const { inputs, maxEpoch, userSignature } = parseZkLoginSignature(signatureBytes);
	const { issBase64Details, addressSeed } = inputs;
	const iss = extractClaimValue<string>(issBase64Details, 'iss');
	const publicIdentifer = toZkLoginPublicIdentifier(BigInt(addressSeed), iss);
	return {
		serializedSignature: toBase64(bytes),
		signatureScheme: 'ZkLogin' as const,
		zkLogin: {
			inputs,
			maxEpoch,
			userSignature,
			iss,
			addressSeed: BigInt(addressSeed),
		},
		signature: bytes,
		publicKey: publicIdentifer.toRawBytes(),
	};
}
