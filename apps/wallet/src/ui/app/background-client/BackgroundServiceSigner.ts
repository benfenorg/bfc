// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiClient } from '@benfen/bfc.js/client';
import type { SerializedSignature } from '@benfen/bfc.js/cryptography';

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
