// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, it } from 'vitest';

import { BCS, getBenfenMoveConfig } from '../../../src/bcs/src/index.js';

describe('parseTypeName', () => {
	it('parses nested struct type from a string', () => {
		const bcs = new BCS(getBenfenMoveConfig());

		const type =
			'0x5::foo::Foo<0x5::bar::Bar, 0x6::amm::LP<0x2::bfc::BFC, 0x7::example_coin::EXAMPLE_COIN>>';
		expect(bcs.parseTypeName(type)).toEqual({
			name: '0x5::foo::Foo',
			params: ['0x5::bar::Bar', '0x6::amm::LP<0x2::bfc::BFC, 0x7::example_coin::EXAMPLE_COIN>'],
		});
	});
});
