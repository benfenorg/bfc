// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';
import type { BenfenTransactionBlockResponse } from '@benfen/bfc.js/client';
import { type SignedTransaction } from '@benfen/bfc.js/wallet-standard';

export interface ExecuteTransactionResponse extends BasePayload {
	type: 'execute-transaction-response';
	result: BenfenTransactionBlockResponse & SignedTransaction;
}

export function isExecuteTransactionResponse(
	payload: Payload,
): payload is ExecuteTransactionResponse {
	return isBasePayload(payload) && payload.type === 'execute-transaction-response';
}

export interface SignTransactionResponse extends BasePayload {
	type: 'sign-transaction-response';
	result: SignedTransaction;
}

export function isSignTransactionResponse(payload: Payload): payload is SignTransactionResponse {
	return isBasePayload(payload) && payload.type === 'sign-transaction-response';
}
