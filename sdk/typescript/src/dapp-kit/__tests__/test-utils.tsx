// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import type { ComponentProps } from 'react';

import { BenfenClient, getFullnodeUrl } from '../../client/index.js';
import { getWallets } from '../../wallet-standard/index.js';
import type { IdentifierRecord, ReadonlyWalletAccount } from '../../wallet-standard/index.js';
import { BenfenClientProvider, WalletProvider } from '../index.js';
import { createMockAccount } from './mocks/mockAccount.js';
import { MockWallet } from './mocks/mockWallet.js';

export function createBenfenClientContextWrapper(client: BenfenClient) {
	return function BenfenClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <BenfenClientProvider networks={{ test: client }}>{children}</BenfenClientProvider>;
	};
}

export function createWalletProviderContextWrapper(
	providerProps: Omit<ComponentProps<typeof WalletProvider>, 'children'> = {},
	benfenClient: BenfenClient = new BenfenClient({ url: getFullnodeUrl('localnet') }),
) {
	const queryClient = new QueryClient();
	return function WalletProviderContextWrapper({ children }: { children: React.ReactNode }) {
		return (
			<BenfenClientProvider networks={{ test: benfenClient }}>
				<QueryClientProvider client={queryClient}>
					<WalletProvider {...providerProps}>{children}</WalletProvider>;
				</QueryClientProvider>
			</BenfenClientProvider>
		);
	};
}

export function registerMockWallet({
	id,
	walletName,
	accounts = [createMockAccount()],
	features = {},
}: {
	id?: string | null;
	walletName: string;
	accounts?: ReadonlyWalletAccount[];
	features?: IdentifierRecord<unknown>;
}) {
	const walletsApi = getWallets();
	const mockWallet = new MockWallet(id ?? crypto.randomUUID(), walletName, accounts, features);

	return {
		unregister: walletsApi.register(mockWallet),
		mockWallet,
	};
}
