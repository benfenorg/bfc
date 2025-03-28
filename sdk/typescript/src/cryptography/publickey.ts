// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';

import { bcs } from '../bcs/index.js';
import { toB64 } from '../bcs/src/index.js';
import { BENFEN_ADDRESS_LENGTH, normalizeHexAddress } from '../utils/bf-types.js';
import type { SerializedSignature } from './index.js';
import { IntentScope, messageWithIntent } from './intent.js';

/**
 * Value to be converted into public key.
 */
export type PublicKeyInitData = string | Uint8Array | Iterable<number>;

export function bytesEqual(a: Uint8Array, b: Uint8Array) {
	if (a === b) return true;

	if (a.length !== b.length) {
		return false;
	}

	for (let i = 0; i < a.length; i++) {
		if (a[i] !== b[i]) {
			return false;
		}
	}
	return true;
}

/**
 * A public key
 */
export abstract class PublicKey {
	/**
	 * Checks if two public keys are equal
	 */
	equals(publicKey: PublicKey) {
		return bytesEqual(this.toRawBytes(), publicKey.toRawBytes());
	}

	/**
	 * Return the base-64 representation of the public key
	 */
	toBase64() {
		return toB64(this.toRawBytes());
	}

	toString(): never {
		throw new Error(
			'`toString` is not implemented on public keys. Use `toBase64()` or `toRawBytes()` instead.',
		);
	}

	/**
	 * Return the Benfen representation of the public key encoded in
	 * base-64. A Benfen public key is formed by the concatenation
	 * of the scheme flag with the raw bytes of the public key
	 */
	toBenfenPublicKey(): string {
		const bytes = this.toBenfenBytes();
		return toB64(bytes);
	}

	verifyWithIntent(
		bytes: Uint8Array,
		signature: Uint8Array | SerializedSignature,
		intent: IntentScope,
	): Promise<boolean> {
		const intentMessage = messageWithIntent(intent, bytes);
		const digest = blake2b(intentMessage, { dkLen: 32 });

		return this.verify(digest, signature);
	}

	/**
	 * Verifies that the signature is valid for for the provided PersonalMessage
	 */
	verifyPersonalMessage(
		message: Uint8Array,
		signature: Uint8Array | SerializedSignature,
	): Promise<boolean> {
		return this.verifyWithIntent(
			bcs.vector(bcs.u8()).serialize(message).toBytes(),
			signature,
			IntentScope.PersonalMessage,
		);
	}

	/**
	 * Verifies that the signature is valid for for the provided TransactionBlock
	 */
	verifyTransactionBlock(
		transactionBlock: Uint8Array,
		signature: Uint8Array | SerializedSignature,
	): Promise<boolean> {
		return this.verifyWithIntent(transactionBlock, signature, IntentScope.TransactionData);
	}

	/**
	 * Returns the bytes representation of the public key
	 * prefixed with the signature scheme flag
	 */
	toBenfenBytes(): Uint8Array {
		const rawBytes = this.toRawBytes();
		const benfenBytes = new Uint8Array(rawBytes.length + 1);
		benfenBytes.set([this.flag()]);
		benfenBytes.set(rawBytes, 1);

		return benfenBytes;
	}

	/**
	 * Return the Benfen address associated with this Ed25519 public key
	 */
	toHexAddress(): string {
		// Each hex char represents half a byte, hence hex address doubles the length
		return normalizeHexAddress(
			bytesToHex(blake2b(this.toBenfenBytes(), { dkLen: 32 })).slice(0, BENFEN_ADDRESS_LENGTH * 2),
		);
	}

	/**
	 * Return the byte array representation of the public key
	 */
	abstract toRawBytes(): Uint8Array;

	/**
	 * Return signature scheme flag of the public key
	 */
	abstract flag(): number;

	/**
	 * Verifies that the signature is valid for for the provided message
	 */
	abstract verify(data: Uint8Array, signature: Uint8Array | SerializedSignature): Promise<boolean>;
}
