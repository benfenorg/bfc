// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, test } from 'vitest';

import { isValidBenfenNSName, normalizeBenfenNSName } from '../../../src/utils';

describe('isValidBenfenNSName', () => {
	test('valid BenfenNS names', () => {
		expect(isValidBenfenNSName('example.benfen')).toBe(true);
		expect(isValidBenfenNSName('EXAMPLE.benfen')).toBe(true);
		expect(isValidBenfenNSName('@example')).toBe(true);
		expect(isValidBenfenNSName('1.example.benfen')).toBe(true);
		expect(isValidBenfenNSName('1@example')).toBe(true);
		expect(isValidBenfenNSName('a.b.c.example.benfen')).toBe(true);
		expect(isValidBenfenNSName('A.B.c.123@Example')).toBe(true);
		expect(isValidBenfenNSName('1-a@1-b')).toBe(true);
		expect(isValidBenfenNSName('1-a.1-b.benfen')).toBe(true);
		expect(isValidBenfenNSName('-@test')).toBe(false);
		expect(isValidBenfenNSName('-1@test')).toBe(false);
		expect(isValidBenfenNSName('test@-')).toBe(false);
		expect(isValidBenfenNSName('test@-1')).toBe(false);
		expect(isValidBenfenNSName('test@-a')).toBe(false);
		expect(isValidBenfenNSName('test.benfen2')).toBe(false);
		expect(isValidBenfenNSName('.benfen2')).toBe(false);
		expect(isValidBenfenNSName('test@')).toBe(false);
		expect(isValidBenfenNSName('@@')).toBe(false);
		expect(isValidBenfenNSName('@@test')).toBe(false);
		expect(isValidBenfenNSName('test@test.test')).toBe(false);
		expect(isValidBenfenNSName('@test.test')).toBe(false);
		expect(isValidBenfenNSName('#@test')).toBe(false);
		expect(isValidBenfenNSName('test@#')).toBe(false);
		expect(isValidBenfenNSName('test.#.benfen')).toBe(false);
		expect(isValidBenfenNSName('#.benfen')).toBe(false);
		expect(isValidBenfenNSName('@.test.sue')).toBe(false);
	});
});

describe('normalizeBenfenNSName', () => {
	test('normalize BenfenNS names', () => {
		expect(normalizeBenfenNSName('example.benfen')).toMatch('@example');
		expect(normalizeBenfenNSName('EXAMPLE.benfen')).toMatch('@example');
		expect(normalizeBenfenNSName('@example')).toMatch('@example');
		expect(normalizeBenfenNSName('1.example.benfen')).toMatch('1@example');
		expect(normalizeBenfenNSName('1@example')).toMatch('1@example');
		expect(normalizeBenfenNSName('a.b.c.example.benfen')).toMatch('a.b.c@example');
		expect(normalizeBenfenNSName('A.B.c.123@Example')).toMatch('a.b.c.123@example');
		expect(normalizeBenfenNSName('1-a@1-b')).toMatch('1-a@1-b');
		expect(normalizeBenfenNSName('1-a.1-b.benfen')).toMatch('1-a@1-b');

		expect(normalizeBenfenNSName('example.benfen', 'dot')).toMatch('example.benfen');
		expect(normalizeBenfenNSName('EXAMPLE.benfen', 'dot')).toMatch('example.benfen');
		expect(normalizeBenfenNSName('@example', 'dot')).toMatch('example.benfen');
		expect(normalizeBenfenNSName('1.example.benfen', 'dot')).toMatch('1.example.benfen');
		expect(normalizeBenfenNSName('1@example', 'dot')).toMatch('1.example.benfen');
		expect(normalizeBenfenNSName('a.b.c.example.benfen', 'dot')).toMatch('a.b.c.example.benfen');
		expect(normalizeBenfenNSName('A.B.c.123@Example', 'dot')).toMatch('a.b.c.123.example.benfen');
		expect(normalizeBenfenNSName('1-a@1-b', 'dot')).toMatch('1-a.1-b.benfen');
		expect(normalizeBenfenNSName('1-a.1-b.benfen', 'dot')).toMatch('1-a.1-b.benfen');

		expect(() => normalizeBenfenNSName('-@test')).toThrowError('Invalid BenfenNS name -@test');
		expect(normalizeBenfenNSName('1-a@1-b')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(normalizeBenfenNSName('1-a.1-b.benfen')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(() => normalizeBenfenNSName('-@test')).toThrowError('Invalid BenfenNS name -@test');
		expect(() => normalizeBenfenNSName('-1@test')).toThrowError('Invalid BenfenNS name -1@test');
		expect(() => normalizeBenfenNSName('test@-')).toThrowError('Invalid BenfenNS name test@-');
		expect(() => normalizeBenfenNSName('test@-1')).toThrowError('Invalid BenfenNS name test@-1');
		expect(() => normalizeBenfenNSName('test@-a')).toThrowError('Invalid BenfenNS name test@-a');
		expect(() => normalizeBenfenNSName('test.benfen2')).toThrowError(
			'Invalid BenfenNS name test.benfen2',
		);
		expect(() => normalizeBenfenNSName('.benfen2')).toThrowError('Invalid BenfenNS name .benfen2');
		expect(() => normalizeBenfenNSName('test@')).toThrowError('Invalid BenfenNS name test@');
		expect(() => normalizeBenfenNSName('@@')).toThrowError('Invalid BenfenNS name @@');
		expect(() => normalizeBenfenNSName('@@test')).toThrowError('Invalid BenfenNS name @@test');
		expect(() => normalizeBenfenNSName('test@test.test')).toThrowError(
			'Invalid BenfenNS name test@test.test',
		);
		expect(() => normalizeBenfenNSName('@test.test')).toThrowError(
			'Invalid BenfenNS name @test.test',
		);
		expect(() => normalizeBenfenNSName('#@test')).toThrowError('Invalid BenfenNS name #@test');
		expect(() => normalizeBenfenNSName('test@#')).toThrowError('Invalid BenfenNS name test@#');
		expect(() => normalizeBenfenNSName('test.#.benfen')).toThrowError(
			'Invalid BenfenNS name test.#.benfen',
		);
		expect(() => normalizeBenfenNSName('#.benfen')).toThrowError('Invalid BenfenNS name #.benfen');
		expect(() => normalizeBenfenNSName('@.test.sue')).toThrowError(
			'Invalid BenfenNS name @.test.sue',
		);
	});
});
