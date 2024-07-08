// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type BenfenSignMessageOutput } from '@benfen/bfc.js/wallet-standard';

import { isBasePayload, type BasePayload } from '../BasePayload';
import { type Payload } from '../Payload';

export interface SignMessageRequest extends BasePayload {
	type: 'sign-message-request';
	args?: {
		message: string; // base64
		accountAddress: string;
	};
	return?: BenfenSignMessageOutput;
}

export function isSignMessageRequest(payload: Payload): payload is SignMessageRequest {
	return isBasePayload(payload) && payload.type === 'sign-message-request';
}
