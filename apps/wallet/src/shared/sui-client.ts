// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import networkEnv from '_src/background/NetworkEnv';
import { ENV_TO_API, type NetworkEnvType } from '_src/shared/api-env';
import { SuiClient, SuiHTTPTransport } from '@benfen/bfc.js/client';

const suiClientPerNetwork = new Map<string, SuiClient>();

export function getSuiClient({ env, customRpcUrl }: NetworkEnvType): SuiClient {
	const key = `${env}_${customRpcUrl}`;
	if (!suiClientPerNetwork.has(key)) {
		const connection = customRpcUrl ? customRpcUrl : ENV_TO_API[env];
		if (!connection) {
			throw new Error(`API url not found for network env ${env} ${customRpcUrl}`);
		}
		suiClientPerNetwork.set(
			key,
			new SuiClient({
				transport: new SuiHTTPTransport({ url: connection }),
			}),
		);
	}
	return suiClientPerNetwork.get(key)!;
}

export async function getActiveNetworkSuiClient(): Promise<SuiClient> {
	return getSuiClient(await networkEnv.getActiveNetwork());
}
