// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type DelegatedStake } from '@benfen/bfc.js/client';

// Get total Stake SUI for a specific validator address
export const getTokenStakeSuiForValidator = (
	allDelegation: DelegatedStake[],
	validatorAddress?: string | null,
) => {
	return (
		allDelegation.reduce((acc, curr) => {
			if (validatorAddress === curr.validatorAddress) {
				return curr.stakes.reduce((total, { principal }) => total + BigInt(principal), acc);
			}
			return acc;
		}, 0n) || 0n
	);
};
