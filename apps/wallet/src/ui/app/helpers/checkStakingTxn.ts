// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { SuiTransactionBlockResponse } from '@benfen/bfc.js/client';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: SuiTransactionBlockResponse) {
	return false;
}
