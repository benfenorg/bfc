// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD:sdk/typescript/test/unit/bcs/vector.generics.test.ts
import { describe, it, expect } from 'vitest';
import { BCS, getSuiMoveConfig } from '../../../src/bcs/src/index.js';
import { serde } from './utils.js';
=======
import { describe, expect, it } from 'vitest';

import { BCS, getSuiMoveConfig } from '../src/index';
import { serde } from './utils';
>>>>>>> mainnet-v1.24.1:sdk/bcs/tests/vector.generics.test.ts

describe('BCS: Inline struct definitions', () => {
	it('should de/serialize inline definition', () => {
		const bcs = new BCS(getSuiMoveConfig());

		// reported by kklas: vector<T> returns [undefined]
		bcs.registerStructType(['FooType', 'T'], {
			generic_vec: ['vector', 'T'],
		});

		const value = { generic_vec: ['1', '2', '3'] };
		expect(serde(bcs, ['FooType', 'u64'], value)).toEqual(value);
	});
});
