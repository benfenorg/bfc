// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { DelegatedStake } from '@benfen/bfc.js/client';

// Helper function to get the delegation by stakedSuiId
export const getDelegationDataByStakeId = (
	delegationsStake: DelegatedStake[],
	stakeSuiId: string,
) => {
	let stake = null;
	for (const { stakes } of delegationsStake) {
		stake = stakes.find(({ stakedBfcId }) => stakedBfcId === stakeSuiId) || null;
		if (stake) return stake;
	}

	return stake;
};
