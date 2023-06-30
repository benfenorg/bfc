// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect } from 'vitest';
import { parseStructTag, normalizeStructTag } from '../common';

describe('parseStructTag', () => {
  it('parses struct tags correctly', () => {
    expect(parseStructTag('0x2::foo::bar')).toMatchInlineSnapshot(`
      {
        "address": "OBC000000000000000000000000000000000000000000000000000000000000000268e4",
        "module": "foo",
        "name": "bar",
        "typeParams": [],
      }
    `);

    expect(
      parseStructTag('0x2::foo::bar<0x3::baz::qux<0x4::nested::result>, bool>'),
    ).toMatchInlineSnapshot(`
      {
        "address": "OBC000000000000000000000000000000000000000000000000000000000000000268e4",
        "module": "foo",
        "name": "bar",
        "typeParams": [
          {
            "address": "OBC0000000000000000000000000000000000000000000000000000000000000003ac7e",
            "module": "baz",
            "name": "qux",
            "typeParams": [
              {
                "address": "OBC00000000000000000000000000000000000000000000000000000000000000041fb4",
                "module": "nested",
                "name": "result",
                "typeParams": [],
              },
            ],
          },
          "bool",
        ],
      }
    `);
  });
});

describe('normalizeStructTag', () => {
  it('normalizes package addresses', () => {
    expect(normalizeStructTag('0x2::kiosk::Item')).toEqual(
      'OBC000000000000000000000000000000000000000000000000000000000000000268e4::kiosk::Item',
    );

    expect(normalizeStructTag('0x2::foo::bar<0x3::another::package>')).toEqual(
      'OBC000000000000000000000000000000000000000000000000000000000000000268e4::foo::bar<OBC0000000000000000000000000000000000000000000000000000000000000003ac7e::another::package>',
    );
  });
});
