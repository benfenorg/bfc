// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { act, renderHook, waitFor } from '@testing-library/react';
import type { Mock } from 'vitest';

import { BenfenClient, getFullnodeUrl } from '../../../client/index.js';
import { TransactionBlock } from '../../../transactions/index.js';
import {
	WalletFeatureNotSupportedError,
	WalletNotConnectedError,
} from '../../errors/walletErrors.js';
import { useConnectWallet } from '../../hooks/wallet/useConnectWallet.js';
import { useSignAndExecuteTransactionBlock } from '../../hooks/wallet/useSignAndExecuteTransactionBlock.js';
import { benfenFeatures } from '../mocks/mockFeatures.js';
import { createWalletProviderContextWrapper, registerMockWallet } from '../test-utils.js';

describe('useSignAndExecuteTransactionBlock', () => {
	test('throws an error when trying to sign and execute a transaction block without a wallet connection', async () => {
		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(() => useSignAndExecuteTransactionBlock(), { wrapper });

		result.current.mutate({ transactionBlock: new TransactionBlock(), chain: 'bfc:testnet' });

		await waitFor(() => expect(result.current.error).toBeInstanceOf(WalletNotConnectedError));
	});

	test('throws an error when trying to sign and execute a transaction block with a wallet that lacks feature support', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
		});

		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				useSignAndExecuteTransactionBlock: useSignAndExecuteTransactionBlock(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });
		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

		result.current.useSignAndExecuteTransactionBlock.mutate({
			transactionBlock: new TransactionBlock(),
			chain: 'bfc:testnet',
		});
		await waitFor(() =>
			expect(result.current.useSignAndExecuteTransactionBlock.error).toBeInstanceOf(
				WalletFeatureNotSupportedError,
			),
		);

		act(() => unregister());
	});

	test('signing and executing a transaction block from the currently connected account works successfully', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
			features: benfenFeatures,
		});

		const benfenClient = new BenfenClient({ url: getFullnodeUrl('localnet') });
		const executeTransactionBlock = vi.spyOn(benfenClient, 'executeTransactionBlock');

		executeTransactionBlock.mockReturnValueOnce(Promise.resolve({ digest: '123' }));

		const wrapper = createWalletProviderContextWrapper({}, benfenClient);
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				useSignAndExecuteTransactionBlock: useSignAndExecuteTransactionBlock(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });

		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

		const signTransactionBlockFeature = mockWallet.features['bfc:signTransactionBlock'];
		const signTransactionBlockMock = signTransactionBlockFeature!.signTransactionBlock as Mock;

		signTransactionBlockMock.mockReturnValueOnce({
			transactionBlockBytes: 'abc',
			signature: '123',
		});

		result.current.useSignAndExecuteTransactionBlock.mutate({
			transactionBlock: new TransactionBlock(),
			chain: 'bfc:testnet',
		});

		await waitFor(() =>
			expect(result.current.useSignAndExecuteTransactionBlock.isSuccess).toBe(true),
		);
		expect(result.current.useSignAndExecuteTransactionBlock.data).toStrictEqual({
			digest: '123',
		});
		expect(benfenClient.executeTransactionBlock).toHaveBeenCalledWith({
			transactionBlock: 'abc',
			signature: '123',
		});

		act(() => unregister());
	});
});
