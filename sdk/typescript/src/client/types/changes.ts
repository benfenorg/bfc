// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { BenfenObjectChange } from './generated.js';

export type BenfenObjectChangePublished = Extract<BenfenObjectChange, { type: 'published' }>;
export type BenfenObjectChangeTransferred = Extract<BenfenObjectChange, { type: 'transferred' }>;
export type BenfenObjectChangeMutated = Extract<BenfenObjectChange, { type: 'mutated' }>;
export type BenfenObjectChangeDeleted = Extract<BenfenObjectChange, { type: 'deleted' }>;
export type BenfenObjectChangeWrapped = Extract<BenfenObjectChange, { type: 'wrapped' }>;
export type BenfenObjectChangeCreated = Extract<BenfenObjectChange, { type: 'created' }>;
