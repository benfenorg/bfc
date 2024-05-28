// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { type SuiSignMessageOutput } from '@benfen/bfc.js/wallet-standard';

import { isBasePayload } from '_payloads';

import type { SignedTransaction } from '@benfen/bfc.js';
import type { SuiTransactionBlockResponse } from '@benfen/bfc.js/client';
=======
import { isBasePayload } from '_payloads';
>>>>>>> mainnet-v1.24.1
import type { BasePayload, Payload } from '_payloads';
import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { SuiTransactionBlockResponse } from '@mysten/sui.js/client';
import { type SuiSignMessageOutput } from '@mysten/wallet-standard';

export interface TransactionRequestResponse extends BasePayload {
	type: 'transaction-request-response';
	txID: string;
	approved: boolean;
	txResult?: SuiTransactionBlockResponse | SuiSignMessageOutput;
	txResultError?: string;
	txSigned?: SignedTransaction;
}

export function isTransactionRequestResponse(
	payload: Payload,
): payload is TransactionRequestResponse {
	return isBasePayload(payload) && payload.type === 'transaction-request-response';
}
