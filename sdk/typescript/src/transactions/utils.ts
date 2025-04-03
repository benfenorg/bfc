// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { is } from 'valibot';

import type { BenfenMoveNormalizedType } from '../client/index.js';
import { normalizeHexAddress } from '../utils/bf-types.js';
import { Argument } from './data/internal.js';
import type { CallArg } from './data/internal.js';

export function extractMutableReference(
	normalizedType: BenfenMoveNormalizedType,
): BenfenMoveNormalizedType | undefined {
	return typeof normalizedType === 'object' && 'MutableReference' in normalizedType
		? normalizedType.MutableReference
		: undefined;
}

export function extractReference(
	normalizedType: BenfenMoveNormalizedType,
): BenfenMoveNormalizedType | undefined {
	return typeof normalizedType === 'object' && 'Reference' in normalizedType
		? normalizedType.Reference
		: undefined;
}

export function extractStructTag(
	normalizedType: BenfenMoveNormalizedType,
): Extract<BenfenMoveNormalizedType, { Struct: unknown }> | undefined {
	if (typeof normalizedType === 'object' && 'Struct' in normalizedType) {
		return normalizedType;
	}

	const ref = extractReference(normalizedType);
	const mutRef = extractMutableReference(normalizedType);

	if (typeof ref === 'object' && 'Struct' in ref) {
		return ref;
	}

	if (typeof mutRef === 'object' && 'Struct' in mutRef) {
		return mutRef;
	}
	return undefined;
}

export function getIdFromCallArg(arg: string | CallArg) {
	if (typeof arg === 'string') {
		return normalizeHexAddress(arg);
	}

	if (arg.Object) {
		if (arg.Object.ImmOrOwnedObject) {
			return normalizeHexAddress(arg.Object.ImmOrOwnedObject.objectId);
		}

		if (arg.Object.Receiving) {
			return normalizeHexAddress(arg.Object.Receiving.objectId);
		}

		return normalizeHexAddress(arg.Object.SharedObject.objectId);
	}

	if (arg.UnresolvedObject) {
		return normalizeHexAddress(arg.UnresolvedObject.objectId);
	}

	return undefined;
}

export function isArgument(value: unknown): value is Argument {
	return is(Argument, value);
}
