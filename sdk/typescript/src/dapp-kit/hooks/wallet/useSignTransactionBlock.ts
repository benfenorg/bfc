// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';

import type { Transaction } from '../../../transactions/index.js';
import { signTransaction } from '../../../wallet-standard/index.js';
import type {
	BenfenSignTransactionInput,
	SignedTransaction,
} from '../../../wallet-standard/index.js';
import { walletMutationKeys } from '../../constants/walletMutationKeys.js';
import {
	WalletFeatureNotSupportedError,
	WalletNoAccountSelectedError,
	WalletNotConnectedError,
} from '../../errors/walletErrors.js';
import type { PartialBy } from '../../types/utilityTypes.js';
import { useBenfenClient } from '../useBenfenClient.js';
import { useCurrentAccount } from './useCurrentAccount.js';
import { useCurrentWallet } from './useCurrentWallet.js';
import { useReportTransactionEffects } from './useReportTransactionEffects.js';

type UseSignTransactionArgs = PartialBy<
	Omit<BenfenSignTransactionInput, 'transaction'>,
	'account' | 'chain'
> & {
	transaction: Transaction | string;
};

interface UseSignTransactionResult extends SignedTransaction {
	reportTransactionEffects: (effects: string) => void;
}

type UseSignTransactionError =
	| WalletFeatureNotSupportedError
	| WalletNoAccountSelectedError
	| WalletNotConnectedError
	| Error;

type UseSignTransactionMutationOptions = Omit<
	UseMutationOptions<
		UseSignTransactionResult,
		UseSignTransactionError,
		UseSignTransactionArgs,
		unknown
	>,
	'mutationFn'
>;

/**
 * Mutation hook for prompting the user to sign a transaction.
 */
export function useSignTransaction({
	mutationKey,
	...mutationOptions
}: UseSignTransactionMutationOptions = {}): UseMutationResult<
	UseSignTransactionResult,
	UseSignTransactionError,
	UseSignTransactionArgs
> {
	const { currentWallet } = useCurrentWallet();
	const currentAccount = useCurrentAccount();
	const client = useBenfenClient();

	const { mutate: reportTransactionEffects } = useReportTransactionEffects();

	return useMutation({
		mutationKey: walletMutationKeys.signTransaction(mutationKey),
		mutationFn: async ({ transaction, ...signTransactionArgs }) => {
			if (!currentWallet) {
				throw new WalletNotConnectedError('No wallet is connected.');
			}

			const signerAccount = signTransactionArgs.account ?? currentAccount;
			if (!signerAccount) {
				throw new WalletNoAccountSelectedError(
					'No wallet account is selected to sign the transaction with.',
				);
			}

			if (
				!currentWallet.features['bfc:signTransaction'] &&
				!currentWallet.features['bfc:signTransactionBlock']
			) {
				throw new WalletFeatureNotSupportedError(
					"This wallet doesn't support the `signTransaction` feature.",
				);
			}

			const { bytes, signature } = await signTransaction(currentWallet, {
				...signTransactionArgs,
				transaction: {
					toJSON: async () => {
						return typeof transaction === 'string'
							? transaction
							: await transaction.toJSON({
									supportedIntents: [],
									client,
								});
					},
				},
				account: signerAccount,
				chain: signTransactionArgs.chain ?? signerAccount.chains[0],
			});

			return {
				bytes,
				signature,
				reportTransactionEffects: (effects) => {
					reportTransactionEffects({
						effects,
						account: signerAccount,
						chain: signTransactionArgs.chain ?? signerAccount.chains[0],
					});
				},
			};
		},
		...mutationOptions,
	});
}
