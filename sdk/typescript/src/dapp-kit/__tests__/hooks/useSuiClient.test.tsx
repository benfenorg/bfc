// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { renderHook } from '@testing-library/react';

import { BenfenClient, getFullnodeUrl } from '../../../client/index.js';
import { useBenfenClient } from '../../index.js';
import { createBenfenClientContextWrapper } from '../test-utils.js';

describe('useBenfenClient', () => {
	test('throws without a BenfenClientContext', () => {
		expect(() => renderHook(() => useBenfenClient())).toThrowError(
			'Could not find BenfenClientContext. Ensure that you have set up the BenfenClientProvider',
		);
	});

	test('returns a BenfenClient', () => {
		const benfenClient = new BenfenClient({ url: getFullnodeUrl('localnet') });
		const wrapper = createBenfenClientContextWrapper(benfenClient);
		const { result } = renderHook(() => useBenfenClient(), { wrapper });

		expect(result.current).toBe(benfenClient);
	});
});
