// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { type SuiClient } from '@benfen/bfc.js/client';
import { WalletSigner } from '../WalletSigner';

import type { BackgroundClient } from '.';
import type { SerializedSignature } from '@benfen/bfc.js/cryptography';
=======
import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { type SuiClient } from '@mysten/sui.js/client';
import type { SerializedSignature } from '@mysten/sui.js/cryptography';
>>>>>>> mainnet-v1.24.1

import type { BackgroundClient } from '.';
import { WalletSigner } from '../WalletSigner';

export class BackgroundServiceSigner extends WalletSigner {
	readonly #address: string;
	readonly #backgroundClient: BackgroundClient;

	constructor(address: string, backgroundClient: BackgroundClient, client: SuiClient) {
		super(client);
		this.#address = address;
		this.#backgroundClient = backgroundClient;
	}

	async getAddress(): Promise<string> {
		return this.#address;
	}

	signData(data: Uint8Array): Promise<SerializedSignature> {
		return this.#backgroundClient.signData(this.#address, data);
	}

	connect(client: SuiClient) {
		return new BackgroundServiceSigner(this.#address, this.#backgroundClient, client);
	}
}
