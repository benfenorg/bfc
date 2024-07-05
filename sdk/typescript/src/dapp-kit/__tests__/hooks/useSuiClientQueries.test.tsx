// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { renderHook, waitFor } from '@testing-library/react';

import { BenfenClient, getFullnodeUrl } from '../../../client/index.js';
import { useBenfenClientQueries } from '../../hooks/useBenfenClientQueries.js';
import { createWalletProviderContextWrapper } from '../test-utils.js';

const MOCK_GET_All_BALANCE_RESULT_DATA = [
	{
		coinType: '0x2::bfc::BFC',
		coinObjectCount: 1,
		totalBalance: '100000',
		lockedBalance: {},
	},
];
const MOCK_QUERY_TRANSACTION_BLOCK_RESULT_DATA = {
	data: [{ digest: '0x123' }],
	hasNextPage: true,
	nextCursor: 'page2',
};

describe('useBenfenClientQueries', () => {
	const benfenClient = new BenfenClient({ url: getFullnodeUrl('mainnet') });
	const wrapper = createWalletProviderContextWrapper({}, benfenClient);
	test('should fetch data', async () => {
		const getAllBalances = vi.spyOn(benfenClient, 'getAllBalances');
		const queryTransactionBlocks = vi.spyOn(benfenClient, 'queryTransactionBlocks');

		getAllBalances.mockResolvedValueOnce(MOCK_GET_All_BALANCE_RESULT_DATA);
		queryTransactionBlocks.mockResolvedValueOnce(MOCK_QUERY_TRANSACTION_BLOCK_RESULT_DATA);

		const { result } = renderHook(
			() =>
				useBenfenClientQueries({
					queries: [
						{
							method: 'getAllBalances',
							params: {
								owner: '0x123',
							},
						},
						{
							method: 'queryTransactionBlocks',
							params: {
								filter: {
									FromAddress: '0x123',
								},
							},
						},
					],
				}),
			{ wrapper },
		);

		// getAllBalancesResult
		expect(result.current[0].isLoading).toBe(true);
		expect(result.current[0].isError).toBe(false);
		expect(result.current[0].data).toBe(undefined);

		// queryTransactionBlocksResult
		expect(result.current[1].isLoading).toBe(true);
		expect(result.current[1].isError).toBe(false);
		expect(result.current[1].data).toBe(undefined);

		expect(getAllBalances).toHaveBeenCalledWith({
			owner: '0x123',
		});
		expect(queryTransactionBlocks).toHaveBeenCalledWith({
			filter: {
				FromAddress: '0x123',
			},
		});

		await waitFor(() => expect(result.current[0].isSuccess).toBe(true));
		await waitFor(() => expect(result.current[1].isSuccess).toBe(true));

		// getAllBalancesResult
		expect(result.current[0].isLoading).toBe(false);
		expect(result.current[0].isError).toBe(false);
		expect(result.current[0].data).toEqual(MOCK_GET_All_BALANCE_RESULT_DATA);

		// queryTransactionBlocksResult
		expect(result.current[1].isLoading).toBe(false);
		expect(result.current[1].isError).toBe(false);
		expect(result.current[1].data).toEqual(MOCK_QUERY_TRANSACTION_BLOCK_RESULT_DATA);
	});
	test('should fetch data with combine function', async () => {
		const getAllBalances = vi.spyOn(benfenClient, 'getAllBalances');
		const queryTransactionBlocks = vi.spyOn(benfenClient, 'queryTransactionBlocks');

		getAllBalances.mockResolvedValueOnce(MOCK_GET_All_BALANCE_RESULT_DATA);
		queryTransactionBlocks.mockResolvedValueOnce(MOCK_QUERY_TRANSACTION_BLOCK_RESULT_DATA);

		const { result } = renderHook(
			() =>
				useBenfenClientQueries({
					queries: [
						{
							method: 'getAllBalances',
							params: {
								owner: '0x123',
							},
							options: {
								queryKey: ['test#2'],
							},
						},
						{
							method: 'queryTransactionBlocks',
							params: {
								filter: {
									FromAddress: '0x123',
								},
							},
							options: {
								queryKey: ['test#2'],
							},
						},
					],
					combine: (result) => {
						return {
							data: result.map((res) => res.data),
							isSuccess: result.every((res) => res.isSuccess),
							isLoading: result.some((res) => res.isLoading),
							isError: result.some((res) => res.isError),
						};
					},
				}),
			{ wrapper },
		);

		expect(result.current.isLoading).toBe(true);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toStrictEqual([undefined, undefined]);

		expect(getAllBalances).toHaveBeenCalledWith({
			owner: '0x123',
		});
		expect(queryTransactionBlocks).toHaveBeenCalledWith({
			filter: {
				FromAddress: '0x123',
			},
		});

		await waitFor(() => expect(result.current.isSuccess).toBe(true));

		// All request settle
		expect(result.current.isLoading).toBe(false);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toEqual([
			MOCK_GET_All_BALANCE_RESULT_DATA,
			MOCK_QUERY_TRANSACTION_BLOCK_RESULT_DATA,
		]);
	});
});
