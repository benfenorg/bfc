// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { renderHook, waitFor } from '@testing-library/react';

import { BenfenClient, getFullnodeUrl } from '../../../client/index.js';
import { useBenfenClientQuery } from '../../hooks/useBenfenClientQuery.js';
import { createWalletProviderContextWrapper } from '../test-utils.js';

describe('useBenfenClientQuery', () => {
	it('should fetch data', async () => {
		const benfenClient = new BenfenClient({ url: getFullnodeUrl('mainnet') });
		const wrapper = createWalletProviderContextWrapper({}, benfenClient);

		const queryTransactionBlocks = vi.spyOn(benfenClient, 'queryTransactionBlocks');

		queryTransactionBlocks.mockResolvedValueOnce({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});

		const { result } = renderHook(
			() =>
				useBenfenClientQuery('queryTransactionBlocks', {
					filter: {
						FromAddress: '0x123',
					},
				}),
			{ wrapper },
		);

		expect(result.current.isLoading).toBe(true);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toBe(undefined);
		expect(queryTransactionBlocks).toHaveBeenCalledWith({
			filter: {
				FromAddress: '0x123',
			},
		});

		await waitFor(() => expect(result.current.isSuccess).toBe(true));

		expect(result.current.isLoading).toBe(false);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toEqual({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});
	});
});
