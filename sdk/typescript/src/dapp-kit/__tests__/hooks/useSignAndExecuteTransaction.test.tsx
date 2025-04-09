// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { act, renderHook, waitFor } from '@testing-library/react';
import { expect, type Mock } from 'vitest';

import { BenfenClient, getFullnodeUrl } from '../../../client/index.js';
import { Transaction } from '../../../transactions/index.js';
import {
	WalletFeatureNotSupportedError,
	WalletNotConnectedError,
} from '../../errors/walletErrors.js';
import { useConnectWallet, useSignAndExecuteTransaction } from '../../index.js';
import { benfenFeatures } from '../mocks/mockFeatures.js';
import { createWalletProviderContextWrapper, registerMockWallet } from '../test-utils.js';

describe('useSignAndExecuteTransaction', () => {
	test('throws an error when trying to sign and execute a transaction without a wallet connection', async () => {
		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(() => useSignAndExecuteTransaction(), { wrapper });

		result.current.mutate({ transaction: new Transaction(), chain: 'bfc:testnet' });

		await waitFor(() => expect(result.current.error).toBeInstanceOf(WalletNotConnectedError));
	});

	test('throws an error when trying to sign and execute a transaction with a wallet that lacks feature support', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
		});

		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				useSignAndExecuteTransaction: useSignAndExecuteTransaction(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });
		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

		result.current.useSignAndExecuteTransaction.mutate({
			transaction: new Transaction(),
			chain: 'bfc:testnet',
		});
		await waitFor(() =>
			expect(result.current.useSignAndExecuteTransaction.error).toBeInstanceOf(
				WalletFeatureNotSupportedError,
			),
		);

		act(() => unregister());
	});

	test('signing and executing a transaction from the currently connected account works successfully', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
			features: benfenFeatures,
		});

		const benfenClient = new BenfenClient({ url: getFullnodeUrl('localnet') });
		const mockSignMessageFeature = mockWallet.features['bfc:signTransaction'];
		const signTransaction = mockSignMessageFeature!.signTransaction as Mock;

		signTransaction.mockReturnValueOnce({
			bytes: 'abc',
			signature: '123',
		});

		const executeTransaction = vi.spyOn(benfenClient, 'executeTransactionBlock');

		executeTransaction.mockResolvedValueOnce({
			digest: '123',
			rawEffects: [10, 20, 30],
		});

		const wrapper = createWalletProviderContextWrapper({}, benfenClient);
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				useSignAndExecuteTransaction: useSignAndExecuteTransaction(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });

		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

		const signTransactionFeature = mockWallet.features['bfc:signTransaction'];
		const signTransactionMock = signTransactionFeature!.signTransaction as Mock;

		signTransactionMock.mockReturnValueOnce({
			transactionBytes: 'abc',
			signature: '123',
		});

		result.current.useSignAndExecuteTransaction.mutate({
			transaction: new Transaction(),
			chain: 'bfc:testnet',
		});

		await waitFor(() => expect(result.current.useSignAndExecuteTransaction.isSuccess).toBe(true));
		expect(result.current.useSignAndExecuteTransaction.data).toStrictEqual({
			bytes: 'abc',
			digest: '123',
			effects: 'ChQe',
			signature: '123',
			rawEffects: [10, 20, 30],
		});

		const call = signTransaction.mock.calls[0];

		expect(call[0].account).toStrictEqual(mockWallet.accounts[0]);
		expect(call[0].chain).toBe('bfc:testnet');
		expect(await call[0].transaction.toJSON()).toEqual(await new Transaction().toJSON());

		act(() => unregister());
	});

	test('executing with custom data resolver', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
			features: benfenFeatures,
		});

		const benfenClient = new BenfenClient({ url: getFullnodeUrl('localnet') });
		const mockSignMessageFeature = mockWallet.features['bfc:signTransaction'];
		const signTransaction = mockSignMessageFeature!.signTransaction as Mock;

		signTransaction.mockReturnValueOnce({
			bytes: 'abc',
			signature: '123',
		});

		const wrapper = createWalletProviderContextWrapper({}, benfenClient);

		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				useSignAndExecuteTransaction: useSignAndExecuteTransaction({
					execute: async () => ({
						digest: '123',
						custom: 123,
					}),
				}),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });

		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

		const signTransactionFeature = mockWallet.features['bfc:signTransaction'];
		const signTransactionMock = signTransactionFeature!.signTransaction as Mock;

		signTransactionMock.mockReturnValueOnce({
			transactionBytes: 'abc',
			signature: '123',
		});

		result.current.useSignAndExecuteTransaction.mutate({
			transaction: new Transaction(),
			chain: 'bfc:testnet',
		});

		await waitFor(() => expect(result.current.useSignAndExecuteTransaction.isSuccess).toBe(true));
		expect(result.current.useSignAndExecuteTransaction.data).toStrictEqual({
			digest: '123',
			custom: 123,
		});
		expect(result.current.useSignAndExecuteTransaction.data?.custom).toBe(123);

		const call = signTransaction.mock.calls[0];

		expect(call[0].account).toStrictEqual(mockWallet.accounts[0]);
		expect(call[0].chain).toBe('bfc:testnet');
		expect(await call[0].transaction.toJSON()).toEqual(await new Transaction().toJSON());

		act(() => unregister());
	});
});
