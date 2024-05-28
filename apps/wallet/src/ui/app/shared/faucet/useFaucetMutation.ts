// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { requestSuiFromFaucetV0 } from '@benfen/bfc.js/faucet';
=======
import { getFaucetRequestStatus, requestSuiFromFaucetV1 } from '@mysten/sui.js/faucet';
>>>>>>> mainnet-v1.24.1
import { useIsMutating, useMutation, type UseMutationOptions } from '@tanstack/react-query';

import { useActiveAccount } from '../../hooks/useActiveAccount';

type UseFaucetMutationOptions = Pick<UseMutationOptions, 'onError'> & {
	host: string | null;
};

const MAX_FAUCET_REQUESTS_STATUS = 20;
const FAUCET_REQUEST_DELAY = 1500;

export function useFaucetMutation(options?: UseFaucetMutationOptions) {
<<<<<<< HEAD
	const address = useActiveAddress();
	const mutationKey = ['faucet-request-tokens', address];
=======
	const activeAccount = useActiveAccount();
	const activeAddress = activeAccount?.address || null;
	const addressToTopUp = options?.address || activeAddress;
	const mutationKey = ['faucet-request-tokens', activeAddress];
>>>>>>> mainnet-v1.24.1
	const mutation = useMutation({
		mutationKey,
		mutationFn: async () => {
			if (!address) {
				throw new Error('Failed, wallet address not found.');
			}
			if (!options?.host) {
				throw new Error('Failed, faucet host not found.');
			}

<<<<<<< HEAD
			const { error, transferredGasObjects } = await requestSuiFromFaucetV0({
				recipient: address,
=======
			const { error, task: taskId } = await requestSuiFromFaucetV1({
				recipient: addressToTopUp,
>>>>>>> mainnet-v1.24.1
				host: options.host,
			});

			if (error || !taskId) {
				throw new Error(error ?? 'Failed, task id not found.');
			}

			let currentStatus = 'INPROGRESS';
			let requestStatusCount = 0;
			while (currentStatus === 'INPROGRESS') {
				const {
					status: { status, transferred_gas_objects },
					error,
				} = await getFaucetRequestStatus({
					host: options.host,
					taskId,
				});

				currentStatus = status;

				if (
					currentStatus === 'DISCARDED' ||
					error ||
					requestStatusCount > MAX_FAUCET_REQUESTS_STATUS
				) {
					throw new Error(error ?? status ?? 'Something went wrong');
				}

				if (currentStatus === 'SUCCEEDED') {
					return transferred_gas_objects?.sent.reduce((total, { amount }) => total + amount, 0);
				}
				requestStatusCount += 1;
				await new Promise((resolve) => setTimeout(resolve, FAUCET_REQUEST_DELAY));
			}

			throw new Error('Something went wrong');
		},
		...options,
	});
	return {
		...mutation,
		/** If the currently-configured endpoint supports faucet and the active account is unlocked */
		enabled: !!options?.host && !!activeAccount && !activeAccount.isLocked,
		/**
		 * is any faucet request in progress across different instances of the mutation
		 */
		isMutating: useIsMutating({ mutationKey }) > 0,
	};
}
