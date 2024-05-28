// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD:sdk/typescript/test/unit/bcs/alias.test.ts
import { describe, it, expect } from 'vitest';
import { BCS, getSuiMoveConfig } from '../../../src/bcs/src/index.js';
import { serde } from './utils.js';
=======
import { describe, expect, it } from 'vitest';

import { BCS, getSuiMoveConfig } from '../src/index';
import { serde } from './utils';
>>>>>>> mainnet-v1.24.1:sdk/bcs/tests/alias.test.ts

describe('BCS: Aliases', () => {
	it('should support type aliases', () => {
		const bcs = new BCS(getSuiMoveConfig());
		const value = 'this is a string';

		bcs.registerAlias('MyString', BCS.STRING);
		expect(serde(bcs, 'MyString', value)).toEqual(value);
	});

	it('should support recursive definitions in structs', () => {
		const bcs = new BCS(getSuiMoveConfig());
		const value = { name: 'Billy' };

		bcs.registerAlias('UserName', BCS.STRING);
		expect(serde(bcs, { name: 'UserName' }, value)).toEqual(value);
	});

	it('should spot recursive definitions', () => {
		const bcs = new BCS(getSuiMoveConfig());
		const value = 'this is a string';

		bcs.registerAlias('MyString', BCS.STRING);
		bcs.registerAlias(BCS.STRING, 'MyString');

		let error = null;
		try {
			serde(bcs, 'MyString', value);
		} catch (e) {
			error = e;
		}

		expect(error).toBeInstanceOf(Error);
	});
});
