// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { BenfenTransactionBlockResponse } from '@benfen/bfc.js/client';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: BenfenTransactionBlockResponse) {
	return false;
}
