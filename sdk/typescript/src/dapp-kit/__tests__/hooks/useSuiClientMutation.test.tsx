// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { act, renderHook, waitFor } from '@testing-library/react';

import { BenfenClient, getFullnodeUrl } from '../../../client/index.js';
import { useBenfenClientMutation } from '../../hooks/useBenfenClientMutation.js';
import { createWalletProviderContextWrapper } from '../test-utils.js';

describe('useBenfenClientMutation', () => {
	it('should fetch data', async () => {
		const benfenClient = new BenfenClient({ url: getFullnodeUrl('mainnet') });
		const wrapper = createWalletProviderContextWrapper({}, benfenClient);

		const queryTransactionBlocks = vi.spyOn(benfenClient, 'queryTransactionBlocks');

		queryTransactionBlocks.mockResolvedValueOnce({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});

		const { result } = renderHook(() => useBenfenClientMutation('queryTransactionBlocks'), {
			wrapper,
		});

		act(() => {
			result.current.mutate({
				filter: {
					FromAddress: '0x123',
				},
			});
		});

		await waitFor(() => expect(result.current.status).toBe('success'));

		expect(queryTransactionBlocks).toHaveBeenCalledWith({
			filter: {
				FromAddress: '0x123',
			},
		});
		expect(result.current.isPending).toBe(false);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toEqual({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});
	});
});
