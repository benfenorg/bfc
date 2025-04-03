// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { ed25519 } from '@noble/curves/ed25519';
import { describe, expect, it } from 'vitest';

import { fromBase64, toBase58 } from '../../../src/bcs/index';
import { decodeBenfenPrivateKey } from '../../../src/cryptography/keypair';
import { Ed25519Keypair } from '../../../src/keypairs/ed25519';
import { Transaction } from '../../../src/transactions';
import { verifyPersonalMessageSignature, verifyTransactionSignature } from '../../../src/verify';

const VALID_SECRET_KEY = 'mdqVWeFekT7pqy5T49+tV12jO0m+ESW7ki4zSU9JiCg=';
const PRIVATE_KEY_SIZE = 32;

const TEST_CASES = [
	[
		'film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm',
		'benfenprivkey1qzegvtc9nef88shvuj2q9ea6ph42sdyulhvp9sfx69ty88kww2r6utu2ny5',
		'0xf9ccb37c04af5fa8f57cce6048a0b41eaf01b9301eef2414f64c0cdf2c9a380a',
	],
	[
		'require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level',
		'benfenprivkey1qzexllxh33475x9ql59x9e60l508s6x5hwvy0s3frhm34zwyc5ysj2p3de3',
		'0x641936c763d8ad9de3657bb7e056b71fbfbb271bd333421b5bebebfe9fcb339f',
	],
	[
		'organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake',
		'benfenprivkey1qr4r2sjlz94u2cg8umsvpsycxku2p9lf3qrw39ch8j2hagndutz7wzzt84c',
		'0x1dc138927fb296136882fdc730a3976da09663223302a783032c2391ee99d004',
	],
];

describe('ed25519-keypair', () => {
	it('new keypair', () => {
		const keypair = new Ed25519Keypair();
		expect(keypair.getPublicKey().toRawBytes().length).toBe(32);
		expect(2).toEqual(2);
	});

	it('create keypair from secret key', () => {
		const secretKey = fromBase64(VALID_SECRET_KEY);
		const keypair = Ed25519Keypair.fromSecretKey(secretKey);
		expect(keypair.getPublicKey().toBase64()).toEqual(
			'Gy9JCW4+Xb0Pz6nAwM2S2as7IVRLNNXdSmXZi4eLmSI=',
		);
	});

	it('create keypair from secret key and mnemonics matches keytool', () => {
		for (const t of TEST_CASES) {
			// Keypair derived from mnemonic.
			const keypair = Ed25519Keypair.deriveKeypair(t[0]);
			expect(keypair.getPublicKey().toHexAddress()).toEqual(t[2]);

			// Decode benfen private key from Bech32 string
			const parsed = decodeBenfenPrivateKey(t[1]);
			const kp = Ed25519Keypair.fromSecretKey(parsed.secretKey);
			expect(kp.getPublicKey().toHexAddress()).toEqual(t[2]);

			// Exported keypair matches the Bech32 encoded secret key.
			const exported = kp.getSecretKey();
			expect(exported).toEqual(t[1]);
		}
	});

	it('generate keypair from random seed', () => {
		const keypair = Ed25519Keypair.fromSecretKey(Uint8Array.from(Array(PRIVATE_KEY_SIZE).fill(8)));
		expect(keypair.getPublicKey().toBase64()).toEqual(
			'E5j2LG0aRXxRumpLXz29L2n8qTIWIY3ImX5Ba9F9k8o=',
		);
	});

	it('signature of data is valid', async () => {
		const keypair = new Ed25519Keypair();
		const signData = new TextEncoder().encode('hello world');
		const signature = await keypair.sign(signData);
		const isValid = ed25519.verify(signature, signData, keypair.getPublicKey().toRawBytes());
		expect(isValid).toBeTruthy();
		expect(keypair.getPublicKey().verify(signData, signature));
	});

	it('incorrect coin type node for ed25519 derivation path', async () => {
		const keypair = Ed25519Keypair.deriveKeypair(TEST_CASES[0][0], `m/44'/728'/0'/0'/0'`);

		const signData = new TextEncoder().encode('hello world');
		const signature = await keypair.sign(signData);
		const isValid = ed25519.verify(signature, signData, keypair.getPublicKey().toRawBytes());
		expect(isValid).toBeTruthy();
	});

	it('incorrect coin type node for ed25519 derivation path', () => {
		expect(() => {
			Ed25519Keypair.deriveKeypair(TEST_CASES[0][0], `m/44'/0'/0'/0'/0'`);
		}).toThrow('Invalid derivation path');
	});

	it('incorrect purpose node for ed25519 derivation path', () => {
		expect(() => {
			Ed25519Keypair.deriveKeypair(TEST_CASES[0][0], `m/54'/728'/0'/0'/0'`);
		}).toThrow('Invalid derivation path');
	});

	it('invalid mnemonics to derive ed25519 keypair', () => {
		expect(() => {
			Ed25519Keypair.deriveKeypair('aaa');
		}).toThrow('Invalid mnemonic');
	});

	it('signs Transactions', async () => {
		const keypair = new Ed25519Keypair();
		const tx = new Transaction();
		tx.setSender(keypair.getPublicKey().toHexAddress());
		tx.setGasPrice(5);
		tx.setGasBudget(100);
		tx.setGasPayment([
			{
				objectId: (Math.random() * 100000).toFixed(0).padEnd(64, '0'),
				version: String((Math.random() * 10000).toFixed(0)),
				digest: toBase58(
					new Uint8Array([
						0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8,
						9, 1, 2,
					]),
				),
			},
		]);

		const bytes = await tx.build();

		const serializedSignature = (await keypair.signTransaction(bytes)).signature;

		expect(await keypair.getPublicKey().verifyTransaction(bytes, serializedSignature)).toEqual(
			true,
		);
		expect(await keypair.getPublicKey().verifyTransaction(bytes, serializedSignature)).toEqual(
			true,
		);
		expect(!!(await verifyTransactionSignature(bytes, serializedSignature))).toEqual(true);
	});

	it('signs PersonalMessages', async () => {
		const keypair = new Ed25519Keypair();
		const message = new TextEncoder().encode('hello world');

		const serializedSignature = (await keypair.signPersonalMessage(message)).signature;

		expect(
			await keypair.getPublicKey().verifyPersonalMessage(message, serializedSignature),
		).toEqual(true);
		expect(
			await keypair.getPublicKey().verifyPersonalMessage(message, serializedSignature),
		).toEqual(true);
		expect(!!(await verifyPersonalMessageSignature(message, serializedSignature))).toEqual(true);
	});
});
