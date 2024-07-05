// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { useMutation } from '@tanstack/react-query';
import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';

import type { BfcSwitchChainInput, BfcSwitchChainOutput } from '../../../wallet-standard/index.js';
import { walletMutationKeys } from '../../constants/walletMutationKeys.js';
import {
	WalletFeatureNotSupportedError,
	WalletNoAccountSelectedError,
	WalletNotConnectedError,
} from '../../errors/walletErrors.js';
import { useCurrentAccount } from './useCurrentAccount.js';
import { useCurrentWallet } from './useCurrentWallet.js';

type UseSwitchChainArgs = BfcSwitchChainInput;

type UseSwitchChainResult = BfcSwitchChainOutput;

type UseSwitchChainError =
	| WalletFeatureNotSupportedError
	| WalletNoAccountSelectedError
	| WalletNotConnectedError;

type UseSwitchChainMutationOptions = Omit<
	UseMutationOptions<UseSwitchChainResult, UseSwitchChainError, UseSwitchChainArgs>,
	'mutationFn'
>;

export function useSwitchChain({
	mutationKey,
	...mutationOptions
}: UseSwitchChainMutationOptions = {}): UseMutationResult<
	UseSwitchChainResult,
	UseSwitchChainError,
	UseSwitchChainArgs
> {
	const { currentWallet } = useCurrentWallet();
	const currentAccount = useCurrentAccount();

	return useMutation({
		mutationKey: walletMutationKeys.switchChain(mutationKey),
		mutationFn: async (switchChainArgs) => {
			if (!currentWallet) {
				throw new WalletNotConnectedError('No wallet is connected.');
			}

			const signerAccount = currentAccount;
			if (!signerAccount) {
				throw new WalletNoAccountSelectedError(
					'No wallet account is selected to switch chain with.',
				);
			}

			const walletFeature = currentWallet.features['bfc:switchChain'];
			if (!walletFeature) {
				throw new WalletFeatureNotSupportedError(
					"This wallet doesn't support the `SwitchChain` feature.",
				);
			}

			return await walletFeature.switchChain({
				chain: switchChainArgs.chain,
			});
		},
		...mutationOptions,
	});
}
