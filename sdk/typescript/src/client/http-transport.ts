// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { RequestManager, HTTPTransport, Client } from '@open-rpc/client-js';
import { PACKAGE_VERSION, TARGETED_RPC_VERSION } from '../version.js';
import type { WebsocketClientOptions } from '../rpc/websocket-client.js';
import { WebsocketClient } from '../rpc/websocket-client.js';

/**
 * An object defining headers to be passed to the RPC server
 */
export type HttpHeaders = { [header: string]: string };

interface SuiHTTPTransportOptions {
	url: string;
	rpc?: {
		headers?: HttpHeaders;
		url?: string;
	};
	websocket?: WebsocketClientOptions & {
		url?: string;
	};
}

export interface SuiTransportRequestOptions {
	method: string;
	params: unknown[];
}

// eslint-disable-next-line @typescript-eslint/ban-types

export interface SuiTransportSubscribeOptions<T> {
	method: string;
	unsubscribe: string;
	params: unknown[];
	onMessage: (event: T) => void;
}

export interface SuiTransport {
	request<T = unknown>(input: SuiTransportRequestOptions): Promise<T>;
	subscribe<T = unknown>(input: SuiTransportSubscribeOptions<T>): Promise<() => Promise<boolean>>;
}

export class SuiHTTPTransport implements SuiTransport {
	private rpcClient: Client;
	private rpcIndexerClient: Client;
	private websocketClient: WebsocketClient;

	constructor({
		url,
		websocket: { url: websocketUrl, ...websocketOptions } = {} as WebsocketClientOptions,
		rpc,
	}: SuiHTTPTransportOptions) {
		const transport = new HTTPTransport(rpc?.url ?? url, {
			headers: {
				'Content-Type': 'application/json',
				'Client-Sdk-Type': 'typescript',
				'Client-Sdk-Version': PACKAGE_VERSION,
				'Client-Target-Api-Version': TARGETED_RPC_VERSION,
				...rpc?.headers,
			},
		});
		const indexerTransport = new HTTPTransport('https://obcindex.openblock.vip', {
			headers: {
				'Content-Type': 'application/json',
				'Client-Sdk-Type': 'typescript',
				'Client-Sdk-Version': PACKAGE_VERSION,
				'Client-Target-Api-Version': TARGETED_RPC_VERSION,
				...rpc?.headers,
			},
		});

		this.rpcClient = new Client(new RequestManager([transport]));
		this.rpcIndexerClient = new Client(new RequestManager([indexerTransport]));
		this.websocketClient = new WebsocketClient(websocketUrl ?? url, websocketOptions);
	}

	async request<T>(input: SuiTransportRequestOptions): Promise<T> {
		console.warn('test server')
		const indexServer = [
			'suix_getNetworkMetrics',
			'suix_getEpochs',
			'suix_getCurrentEpoch',
			'suix_getMoveCallMetrics',
			'suix_getNetworkOverview',
			'suix_getDaoProposals',
		]
		const useIndexer = input?.method && indexServer.includes(input?.method) ? true : false
		return useIndexer ? this.rpcIndexerClient.request(input) : this.rpcClient.request(input);
	}

	async subscribe<T>(input: SuiTransportSubscribeOptions<T>): Promise<() => Promise<boolean>> {
		const unsubscribe = await this.websocketClient.request(input);

		return async () => !!(await unsubscribe());
	}
}
