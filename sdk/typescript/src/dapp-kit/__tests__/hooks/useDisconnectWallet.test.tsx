// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { act, renderHook, waitFor } from '@testing-library/react';

import { WalletNotConnectedError } from '../../errors/walletErrors.js';
import { useConnectWallet } from '../../hooks/wallet/useConnectWallet.js';
import { useCurrentAccount } from '../../hooks/wallet/useCurrentAccount.js';
import { useCurrentWallet } from '../../hooks/wallet/useCurrentWallet.js';
import { useDisconnectWallet } from '../../hooks/wallet/useDisconnectWallet.js';
import { createWalletProviderContextWrapper, registerMockWallet } from '../test-utils.js';

describe('useDisconnectWallet', () => {
	test('that an error is thrown when trying to disconnect with no active connection', async () => {
		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(() => useDisconnectWallet(), { wrapper });

		result.current.mutate();

		await waitFor(() => expect(result.current.error).toBeInstanceOf(WalletNotConnectedError));
	});

	test('that disconnecting works successfully', async () => {
		const { unregister, mockWallet } = registerMockWallet({ walletName: 'Mock Wallet 1' });

		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				disconnectWallet: useDisconnectWallet(),
				currentWallet: useCurrentWallet(),
				currentAccount: useCurrentAccount(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });

		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));
		expect(result.current.currentWallet.isConnected).toBe(true);
		expect(result.current.currentAccount).toBeTruthy();

		result.current.disconnectWallet.mutate();
		await waitFor(() => expect(result.current.disconnectWallet.isSuccess).toBe(true));

		expect(result.current.currentWallet.isDisconnected).toBe(true);
		expect(result.current.currentAccount).toBeFalsy();

		act(() => {
			unregister();
		});
	});
});
