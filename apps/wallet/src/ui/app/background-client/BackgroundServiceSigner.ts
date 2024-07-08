// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { type BenfenClient } from '@benfen/bfc.js/client';
import type { SerializedSignature } from '@benfen/bfc.js/cryptography';

import type { BackgroundClient } from '.';
import { WalletSigner } from '../WalletSigner';

export class BackgroundServiceSigner extends WalletSigner {
	readonly #account: SerializedUIAccount;
	readonly #backgroundClient: BackgroundClient;

	constructor(
		account: SerializedUIAccount,
		backgroundClient: BackgroundClient,
		client: BenfenClient,
	) {
		super(client);
		this.#account = account;
		this.#backgroundClient = backgroundClient;
	}

	async getAddress(): Promise<string> {
		return this.#account.address;
	}

	signData(data: Uint8Array): Promise<SerializedSignature> {
		return this.#backgroundClient.signData(this.#account.id, data);
	}

	connect(client: BenfenClient) {
		return new BackgroundServiceSigner(this.#account, this.#backgroundClient, client);
	}
}
