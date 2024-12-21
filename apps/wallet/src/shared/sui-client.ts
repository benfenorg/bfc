// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import networkEnv from '_src/background/NetworkEnv';
import { ENV_TO_API, type NetworkEnvType } from '_src/shared/api-env';
import { BenfenClient, BenfenHTTPTransport } from '@benfen/bfc.js/client';

const suiClientPerNetwork = new Map<string, BenfenClient>();

export function getSuiClient({ env, customRpcUrl }: NetworkEnvType): BenfenClient {
	const key = `${env}_${customRpcUrl}`;
	if (!suiClientPerNetwork.has(key)) {
		const connection = customRpcUrl ? customRpcUrl : ENV_TO_API[env];
		if (!connection) {
			throw new Error(`API url not found for network env ${env} ${customRpcUrl}`);
		}
		suiClientPerNetwork.set(
			key,
			new BenfenClient({
				transport: new BenfenHTTPTransport({ url: connection }),
			}),
		);
	}
	return suiClientPerNetwork.get(key)!;
}

export async function getActiveNetworkSuiClient(): Promise<BenfenClient> {
	return getSuiClient(await networkEnv.getActiveNetwork());
}
