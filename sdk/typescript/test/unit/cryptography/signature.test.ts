// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll } from 'vitest';
import {
	SerializeSignatureInput,
	toSerializedSignature,
	parseSerializedSignature,
} from '../../../src/cryptography/signature';
import {
	PubkeyWeightPair,
	combinePartialSigs,
	decodeMultiSig,
} from '../../../src/cryptography/multisig';
import { fromB64, toB64 } from '@benfen/bcs';
import { PublicKey } from '../../../src/cryptography/publickey';
import { Ed25519Keypair, Ed25519PublicKey } from '../../../src/keypairs/ed25519';
import { Secp256k1Keypair } from '../../../src/keypairs/secp256k1';
import { Secp256r1Keypair } from '../../../src/keypairs/secp256r1';

describe('Signature', () => {
	let k1: Ed25519Keypair,
		pk1: Ed25519PublicKey,
		k2: Secp256k1Keypair,
		pk2: PublicKey,
		k3: Secp256r1Keypair,
		pk3: PublicKey;

	beforeAll(() => {
		const VALID_SECP256K1_SECRET_KEY = [
			59, 148, 11, 85, 134, 130, 61, 253, 2, 174, 59, 70, 27, 180, 51, 107, 94, 203, 174, 253, 102,
			39, 170, 146, 46, 252, 4, 143, 236, 12, 136, 28,
		];

		const VALID_SECP256R1_SECRET_KEY = [
			66, 37, 141, 205, 161, 76, 241, 17, 198, 2, 184, 151, 27, 140, 200, 67, 233, 30, 70, 202, 144,
			81, 81, 192, 39, 68, 166, 176, 23, 230, 147, 22,
		];

		const secret_key_k1 = new Uint8Array(VALID_SECP256K1_SECRET_KEY);
		const secret_key_r1 = new Uint8Array(VALID_SECP256R1_SECRET_KEY);

		k1 = Ed25519Keypair.fromSecretKey(secret_key_k1);
		pk1 = k1.getPublicKey();

		k2 = Secp256k1Keypair.fromSecretKey(secret_key_k1);
		pk2 = k2.getPublicKey();

		k3 = Secp256r1Keypair.fromSecretKey(secret_key_r1);
		pk3 = k3.getPublicKey();
	});

	it('`toSerializedSignature()` should correctly serialize signature', async () => {
		const pubkeyWeightPairs: PubkeyWeightPair[] = [
			{
				pubKey: pk1,
				weight: 1,
			},
			{
				pubKey: pk2,
				weight: 2,
			},
			{
				pubKey: pk3,
				weight: 3,
			},
		];

		const data = new Uint8Array([0, 0, 0, 5, 72, 101, 108, 108, 111]);

		const sig1 = await k1.signPersonalMessage(data);
		const sig2 = await k2.signPersonalMessage(data);
		const sig3 = await k3.signPersonalMessage(data);

		const multisig = combinePartialSigs(
			[sig1.signature, sig2.signature, sig3.signature],
			pubkeyWeightPairs,
			3,
		);

		const decoded = decodeMultiSig(multisig);
		const SerializeSignatureInput: SerializeSignatureInput[] = [
			{
				signatureScheme: decoded[0].signatureScheme,
				signature: decoded[0].signature,
				publicKey: decoded[0].pubKey,
			},
			{
				signatureScheme: decoded[1].signatureScheme,
				signature: decoded[1].signature,
				publicKey: decoded[1].pubKey,
			},
			{
				signatureScheme: decoded[2].signatureScheme,
				signature: decoded[2].signature,
				publicKey: decoded[2].pubKey,
			},
		];

		const serializedSignature1 = toSerializedSignature(SerializeSignatureInput[0]);
		const serializedSignature2 = toSerializedSignature(SerializeSignatureInput[1]);
		const serializedSignature3 = toSerializedSignature(SerializeSignatureInput[2]);

		expect(serializedSignature1).toEqual(sig1.signature);
		expect(serializedSignature2).toEqual(sig2.signature);
		expect(serializedSignature3).toEqual(sig3.signature);
	});

	it('`toSerializedSignature()` should handle invalid parameters', async () => {
		const pubkeyWeightPairs: PubkeyWeightPair[] = [
			{
				pubKey: pk1,
				weight: 1,
			},
		];

		const data = new Uint8Array([0, 0, 0, 5, 72, 101, 108, 108, 111]);

		const sig1 = await k1.signPersonalMessage(data);

		const multisig = combinePartialSigs([sig1.signature], pubkeyWeightPairs, 3);

		const decoded = decodeMultiSig(multisig);

		const SerializeSignatureInput: SerializeSignatureInput[] = [
			{
				signatureScheme: decoded[0].signatureScheme,
				signature: decoded[0].signature,
			},
		];

		expect(() => toSerializedSignature(SerializeSignatureInput[0])).toThrowError(
			new Error('`publicKey` is required'),
		);
	});

	it('`parseSerializedSignature()` should correctly parse serialized signature', async () => {
		const pubkeyWeightPairs: PubkeyWeightPair[] = [
			{
				pubKey: pk1,
				weight: 1,
			},
			{
				pubKey: pk2,
				weight: 2,
			},
			{
				pubKey: pk3,
				weight: 3,
			},
		];

		const data = new Uint8Array([0, 0, 0, 5, 72, 101, 108, 108, 111]);

		const sig1 = await k1.signPersonalMessage(data);
		const sig2 = await k2.signPersonalMessage(data);
		const sig3 = await k3.signPersonalMessage(data);

		const multisig = combinePartialSigs(
			[sig1.signature, sig2.signature, sig3.signature],
			pubkeyWeightPairs,
			3,
		);

		const parsedSignature = parseSerializedSignature(sig1.signature);
		expect(parsedSignature.serializedSignature).toEqual(sig1.signature);
		expect(parsedSignature.signatureScheme).toEqual(k1.getKeyScheme());

		const parsedMultisigSignature = parseSerializedSignature(multisig);
		expect(parsedMultisigSignature.serializedSignature).toEqual(multisig);
		expect(parsedMultisigSignature.signatureScheme).toEqual('MultiSig');
	});

	it('`parseSerializedSignature()` should handle unsupported schemes', async () => {
		const data = new Uint8Array([0, 0, 0, 5, 72, 101, 108, 108, 111]);

		const sig1 = await k1.signPersonalMessage(data);
		const sig2 = await k2.signPersonalMessage(data);

		let bytes = fromB64(sig1.signature);
		bytes[0] = 0x05;
		let invalidSignature = toB64(bytes);

		expect(() => parseSerializedSignature(invalidSignature)).toThrowError(
			new Error('Unable to parse a zk signature. (not implemented yet)'),
		);

		bytes = fromB64(sig2.signature);
		bytes[0] = 0x07;
		invalidSignature = toB64(bytes);

		expect(() => parseSerializedSignature(invalidSignature)).toThrowError(
			new Error('Unsupported signature scheme'),
		);
	});
});
