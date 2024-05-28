// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import useAppSelector from './useAppSelector';
import { activeAccountSelector } from '../redux/slices/account';
=======
import { useMemo } from 'react';

import { useAccounts } from './useAccounts';
>>>>>>> mainnet-v1.24.1

export function useActiveAccount() {
	return useAppSelector(activeAccountSelector);
}
