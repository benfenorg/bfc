// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type DelegatedStake } from '@benfen/bfc.js/client';

// Get Stake SUI by stakeSuiId
export const getStakeSuiBySuiId = (allDelegation: DelegatedStake[], stakeSuiId?: string | null) => {
	return (
		allDelegation.reduce((acc, curr) => {
			const total = BigInt(
				curr.stakes.find(({ stakedBfcId }) => stakedBfcId === stakeSuiId)?.principal || 0,
			);
			return total + acc;
		}, 0n) || 0n
	);
};
