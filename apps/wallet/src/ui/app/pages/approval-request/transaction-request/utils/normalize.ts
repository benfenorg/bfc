// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { type BenfenMoveNormalizedType } from '@benfen/bfc.js/client';

export interface TypeReference {
	address: string;
	module: string;
	name: string;
	typeArguments: BenfenMoveNormalizedType[];
}

export const TX_CONTEXT_TYPE = '0x2::tx_context::TxContext';

/** Takes a normalized move type and returns the address information contained within it */
export function unwrapTypeReference(type: BenfenMoveNormalizedType): null | TypeReference {
	if (typeof type === 'object') {
		if ('Struct' in type) {
			return type.Struct;
		}
		if ('Reference' in type) {
			return unwrapTypeReference(type.Reference);
		}
		if ('MutableReference' in type) {
			return unwrapTypeReference(type.MutableReference);
		}
		if ('Vector' in type) {
			return unwrapTypeReference(type.Vector);
		}
	}
	return null;
}
