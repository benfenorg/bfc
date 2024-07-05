// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
	DisplayFieldsResponse,
	BenfenObjectChange,
	BenfenObjectChangeCreated,
	BenfenObjectChangeDeleted,
	BenfenObjectChangeMutated,
	BenfenObjectChangePublished,
	BenfenObjectChangeTransferred,
	BenfenObjectChangeWrapped,
} from '@benfen/bfc.js/client';

import { groupByOwner } from './groupByOwner';
import { SuiObjectChangeTypes } from './types';

export type WithDisplayFields<T> = T & { display?: DisplayFieldsResponse };
export type SuiObjectChangeWithDisplay = WithDisplayFields<BenfenObjectChange>;

export type ObjectChanges = {
	changesWithDisplay: SuiObjectChangeWithDisplay[];
	changes: BenfenObjectChange[];
	ownerType: string;
};
export type ObjectChangesByOwner = Record<string, ObjectChanges>;

export type ObjectChangeSummary = {
	[K in SuiObjectChangeTypes]: ObjectChangesByOwner;
};

export const getObjectChangeSummary = (objectChanges: SuiObjectChangeWithDisplay[]) => {
	if (!objectChanges) return null;

	const mutated = objectChanges.filter(
		(change) => change.type === 'mutated',
	) as BenfenObjectChangeMutated[];

	const created = objectChanges.filter(
		(change) => change.type === 'created',
	) as BenfenObjectChangeCreated[];

	const transferred = objectChanges.filter(
		(change) => change.type === 'transferred',
	) as BenfenObjectChangeTransferred[];

	const published = objectChanges.filter(
		(change) => change.type === 'published',
	) as BenfenObjectChangePublished[];

	const wrapped = objectChanges.filter(
		(change) => change.type === 'wrapped',
	) as BenfenObjectChangeWrapped[];

	const deleted = objectChanges.filter(
		(change) => change.type === 'deleted',
	) as BenfenObjectChangeDeleted[];

	return {
		transferred: groupByOwner(transferred),
		created: groupByOwner(created),
		mutated: groupByOwner(mutated),
		published: groupByOwner(published),
		wrapped: groupByOwner(wrapped),
		deleted: groupByOwner(deleted),
	};
};
