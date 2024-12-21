// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { BenfenTransactionBlockResponse } from '@benfen/bfc.js/client';
import {
	type BenfenSignAndExecuteTransactionBlockInput,
	type BenfenSignMessageOutput,
} from '@benfen/bfc.js/wallet-standard';

export type TransactionDataType = {
	type: 'transaction';
	data: string;
	account: string;
	justSign?: boolean;
	requestType?: BenfenSignAndExecuteTransactionBlockInput['requestType'];
	options?: BenfenSignAndExecuteTransactionBlockInput['options'];
};

export type SignMessageDataType = {
	type: 'sign-message';
	message: string;
	accountAddress: string;
};

export type ApprovalRequest = {
	id: string;
	approved: boolean | null;
	origin: string;
	originFavIcon?: string;
	txResult?: BenfenTransactionBlockResponse | BenfenSignMessageOutput;
	txResultError?: string;
	txSigned?: SignedTransaction;
	createdDate: string;
	tx: TransactionDataType | SignMessageDataType;
};

export interface SignMessageApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
	tx: SignMessageDataType;
	txResult?: BenfenSignMessageOutput;
}

export interface TransactionApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
	tx: TransactionDataType;
	txResult?: BenfenTransactionBlockResponse;
}

export function isSignMessageApprovalRequest(
	request: ApprovalRequest,
): request is SignMessageApprovalRequest {
	return request.tx.type === 'sign-message';
}

export function isTransactionApprovalRequest(
	request: ApprovalRequest,
): request is TransactionApprovalRequest {
	return request.tx.type !== 'sign-message';
}
