// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { SuiObjectChange } from './generated.js';

export type BenfenObjectChangePublished = Extract<SuiObjectChange, { type: 'published' }>;
export type BenfenObjectChangeTransferred = Extract<SuiObjectChange, { type: 'transferred' }>;
export type BenfenObjectChangeMutated = Extract<SuiObjectChange, { type: 'mutated' }>;
export type BenfenObjectChangeDeleted = Extract<SuiObjectChange, { type: 'deleted' }>;
export type BenfenObjectChangeWrapped = Extract<SuiObjectChange, { type: 'wrapped' }>;
export type BenfenObjectChangeCreated = Extract<SuiObjectChange, { type: 'created' }>;
