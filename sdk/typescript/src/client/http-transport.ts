// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { PACKAGE_VERSION, TARGETED_RPC_VERSION } from '../version.js';
import { BenfenHTTPStatusError, JsonRpcError } from './errors.js';
import type { WebsocketClientOptions } from './rpc-websocket-client.js';
import { WebsocketClient } from './rpc-websocket-client.js';

/**
 * An object defining headers to be passed to the RPC server
 */
export type HttpHeaders = { [header: string]: string };

interface BenfenHTTPTransportOptions {
	fetch?: typeof fetch;
	WebSocketConstructor?: typeof WebSocket;
	url: string;
	rpc?: {
		headers?: HttpHeaders;
		url?: string;
	};
	websocket?: WebsocketClientOptions & {
		url?: string;
	};
}

export interface BenfenTransportRequestOptions {
	method: string;
	params: unknown[];
}

// eslint-disable-next-line @typescript-eslint/ban-types

export interface BenfenTransportSubscribeOptions<T> {
	method: string;
	unsubscribe: string;
	params: unknown[];
	onMessage: (event: T) => void;
}

export interface BenfenTransport {
	request<T = unknown>(input: BenfenTransportRequestOptions): Promise<T>;
	subscribe<T = unknown>(
		input: BenfenTransportSubscribeOptions<T>,
	): Promise<() => Promise<boolean>>;
}

export class BenfenHTTPTransport implements BenfenTransport {
	#requestId = 0;
	#options: BenfenHTTPTransportOptions;
	#websocketClient?: WebsocketClient;

	constructor(options: BenfenHTTPTransportOptions) {
		this.#options = options;
	}

	fetch(input: RequestInfo, init?: RequestInit): Promise<Response> {
		const fetch = this.#options.fetch ?? globalThis.fetch;

		if (!fetch) {
			throw new Error(
				'The current environment does not support fetch, you can provide a fetch implementation in the options for BenfenHTTPTransport.',
			);
		}

		return fetch(input, init);
	}

	#getWebsocketClient(): WebsocketClient {
		if (!this.#websocketClient) {
			const WebSocketConstructor = this.#options.WebSocketConstructor ?? globalThis.WebSocket;
			if (!WebSocketConstructor) {
				throw new Error(
					'The current environment does not support WebSocket, you can provide a WebSocketConstructor in the options for BenfenHTTPTransport.',
				);
			}

			this.#websocketClient = new WebsocketClient(
				this.#options.websocket?.url ?? this.#options.url,
				{
					WebSocketConstructor,
					...this.#options.websocket,
				},
			);
		}

		return this.#websocketClient;
	}

	async request<T>(input: BenfenTransportRequestOptions): Promise<T> {
		this.#requestId += 1;

		const res = await this.fetch(this.#options.rpc?.url ?? this.#options.url, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				'Client-Sdk-Type': 'typescript',
				'Client-Sdk-Version': PACKAGE_VERSION,
				'Client-Target-Api-Version': TARGETED_RPC_VERSION,
				...this.#options.rpc?.headers,
			},
			body: JSON.stringify({
				jsonrpc: '2.0',
				id: this.#requestId,
				method: input.method,
				params: input.params,
			}),
		});

		if (!res.ok) {
			throw new BenfenHTTPStatusError(
				`Unexpected status code: ${res.status}`,
				res.status,
				res.statusText,
			);
		}

		const data = await res.json();

		if ('error' in data && data.error != null) {
			throw new JsonRpcError(data.error.message, data.error.code);
		}

		return data.result;
	}

	async subscribe<T>(input: BenfenTransportSubscribeOptions<T>): Promise<() => Promise<boolean>> {
		const unsubscribe = await this.#getWebsocketClient().subscribe(input);

		return async () => !!(await unsubscribe());
	}
}
