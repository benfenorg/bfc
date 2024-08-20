// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransactionBlock.test.tsx
========
import { Transaction } from '@mysten/sui/transactions';
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransaction.test.tsx
import { act, renderHook, waitFor } from '@testing-library/react';
import type { Mock } from 'vitest';

import { TransactionBlock } from '../../../transactions/index.js';
import {
	WalletFeatureNotSupportedError,
	WalletNotConnectedError,
<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransactionBlock.test.tsx
} from '../../errors/walletErrors.js';
import { useConnectWallet } from '../../hooks/wallet/useConnectWallet.js';
import { useSignTransactionBlock } from '../../hooks/wallet/useSignTransactionBlock.js';
import { benfenFeatures } from '../mocks/mockFeatures.js';
========
} from '../../src/errors/walletErrors.js';
import { useConnectWallet, useSignTransaction } from '../../src/index.js';
import { suiFeatures } from '../mocks/mockFeatures.js';
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransaction.test.tsx
import { createWalletProviderContextWrapper, registerMockWallet } from '../test-utils.js';

describe('useSignTransaction', () => {
	test('throws an error when trying to sign a transaction without a wallet connection', async () => {
		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(() => useSignTransaction(), { wrapper });

<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransactionBlock.test.tsx
		result.current.mutate({ transactionBlock: new TransactionBlock(), chain: 'bfc:testnet' });
========
		result.current.mutate({ transaction: new Transaction(), chain: 'sui:testnet' });
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransaction.test.tsx

		await waitFor(() => expect(result.current.error).toBeInstanceOf(WalletNotConnectedError));
	});

	test('throws an error when trying to sign a transaction with a wallet that lacks feature support', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
		});

		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				signTransaction: useSignTransaction(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });
		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransactionBlock.test.tsx
		result.current.signTransactionBlock.mutate({
			transactionBlock: new TransactionBlock(),
			chain: 'bfc:testnet',
========
		result.current.signTransaction.mutate({
			transaction: new Transaction(),
			chain: 'sui:testnet',
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransaction.test.tsx
		});
		await waitFor(() =>
			expect(result.current.signTransaction.error).toBeInstanceOf(WalletFeatureNotSupportedError),
		);

		act(() => unregister());
	});

	test('signing a transaction from the currently connected account works successfully', async () => {
		const { unregister, mockWallet } = registerMockWallet({
			walletName: 'Mock Wallet 1',
			features: benfenFeatures,
		});

		const wrapper = createWalletProviderContextWrapper();
		const { result } = renderHook(
			() => ({
				connectWallet: useConnectWallet(),
				signTransaction: useSignTransaction(),
			}),
			{ wrapper },
		);

		result.current.connectWallet.mutate({ wallet: mockWallet });

		await waitFor(() => expect(result.current.connectWallet.isSuccess).toBe(true));

<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransactionBlock.test.tsx
		const signTransactionBlockFeature = mockWallet.features['bfc:signTransactionBlock'];
		const signTransactionBlockMock = signTransactionBlockFeature!.signTransactionBlock as Mock;
========
		const signTransactionFeature = mockWallet.features['sui:signTransaction'];
		const signTransactionMock = signTransactionFeature!.signTransaction as Mock;
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransaction.test.tsx

		signTransactionMock.mockReturnValueOnce({
			bytes: 'abc',
			signature: '123',
		});

<<<<<<<< HEAD:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransactionBlock.test.tsx
		result.current.signTransactionBlock.mutate({
			transactionBlock: new TransactionBlock(),
			chain: 'bfc:testnet',
========
		result.current.signTransaction.mutate({
			transaction: new Transaction(),
			chain: 'sui:testnet',
>>>>>>>> releases/sui-v1.31.0-release:sdk/typescript/src/dapp-kit/__tests__/hooks/useSignTransaction.test.tsx
		});

		await waitFor(() => expect(result.current.signTransaction.isSuccess).toBe(true));
		expect(result.current.signTransaction.data).toStrictEqual({
			bytes: 'abc',
			signature: '123',
			reportTransactionEffects: expect.any(Function),
		});

		act(() => unregister());
	});
});
