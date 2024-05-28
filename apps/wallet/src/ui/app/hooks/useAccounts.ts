// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useMemo } from 'react';

import useAppSelector from './useAppSelector';
import { accountsAdapterSelectors } from '../redux/slices/account';

export function useAccounts(addressesFilters?: string[]) {
	const accounts = useAppSelector(accountsAdapterSelectors.selectAll);
	return useMemo(() => {
		if (!addressesFilters?.length) {
			return accounts;
		}
		return accounts.filter((anAccount) => addressesFilters.includes(anAccount.address));
	}, [accounts, addressesFilters]);
=======
import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { useQuery } from '@tanstack/react-query';

import { accountsQueryKey } from '../helpers/query-client-keys';
import { useBackgroundClient } from './useBackgroundClient';

export function useAccounts() {
	const backgroundClient = useBackgroundClient();
	return useQuery({
		queryKey: accountsQueryKey,
		queryFn: () => backgroundClient.getStoredEntities<SerializedUIAccount>('accounts'),
		gcTime: 30 * 1000,
		staleTime: 15 * 1000,
		refetchInterval: 30 * 1000,
		meta: { skipPersistedCache: true },
	});
>>>>>>> mainnet-v1.24.1
}
