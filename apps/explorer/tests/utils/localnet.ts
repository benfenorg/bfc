// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// eslint-disable-next-line import/order
import { SuiClient, getFullnodeUrl } from '@benfen/bfc.js/client';
import { type Keypair } from '@benfen/bfc.js/cryptography';
import { Ed25519Keypair } from '@benfen/bfc.js/keypairs/ed25519';
import { TransactionBlock } from '@benfen/bfc.js/transactions';
import 'tsconfig-paths/register';
// eslint-disable-next-line import/order

const addressToKeypair = new Map<string, Keypair>();

export async function split_coin(address: string) {
	const keypair = addressToKeypair.get(address);
	if (!keypair) {
		throw new Error('missing keypair');
	}
	const client = new SuiClient({ url: getFullnodeUrl('localnet') });

	const coins = await client.getCoins({ owner: address });
	const coin_id = coins.data[0].coinObjectId;

	const tx = new TransactionBlock();
	tx.moveCall({
		target: '0x2::pay::split',
		typeArguments: ['0x2::bfc::BFC'],
		arguments: [tx.object(coin_id), tx.pure(10)],
	});

	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showInput: true,
			showEffects: true,
			showEvents: true,
		},
	});

	return result;
}

export async function faucet() {
	const keypair = Ed25519Keypair.generate();
	const address = keypair.getPublicKey().toSuiAddress();
	addressToKeypair.set(address, keypair);
	const res = await fetch('http://127.0.0.1:9123/gas', {
		method: 'POST',
		headers: {
			'content-type': 'application/json',
		},
		body: JSON.stringify({ FixedAmountRequest: { recipient: address } }),
	});
	const data = await res.json();
	if (!res.ok || data.error) {
		throw new Error('Unable to invoke local faucet.');
	}
	return address;
}
