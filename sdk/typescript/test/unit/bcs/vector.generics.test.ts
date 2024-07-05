// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, it } from 'vitest';

import { BCS, getBenfenMoveConfig } from '../../../src/bcs/src/index.js';
import { serde } from './utils.js';

describe('BCS: Inline struct definitions', () => {
	it('should de/serialize inline definition', () => {
		const bcs = new BCS(getBenfenMoveConfig());

		// reported by kklas: vector<T> returns [undefined]
		bcs.registerStructType(['FooType', 'T'], {
			generic_vec: ['vector', 'T'],
		});

		const value = { generic_vec: ['1', '2', '3'] };
		expect(serde(bcs, ['FooType', 'u64'], value)).toEqual(value);
	});
});
