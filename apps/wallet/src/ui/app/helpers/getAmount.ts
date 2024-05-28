// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import type { SuiTransactionBlockKind, TransactionEffects, SuiEvent } from '@benfen/bfc.js/client';
=======
import type { SuiEvent, SuiTransactionBlockKind, TransactionEffects } from '@mysten/sui.js/client';
>>>>>>> mainnet-v1.24.1

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: SuiTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: SuiEvent[],
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
