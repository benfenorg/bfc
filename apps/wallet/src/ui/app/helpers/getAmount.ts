// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type {
	BenfenEvent,
	BenfenTransactionBlockKind,
	TransactionEffects,
} from '@benfen/bfc.js/client';

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: BenfenTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: BenfenEvent[],
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
