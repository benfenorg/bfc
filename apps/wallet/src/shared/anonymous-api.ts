// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import axios from 'axios';

export const anonymousRequest = <TRes, TParams>({
	method,
	params,
}: {
	method: string;
	params: TParams;
}) => {
	return axios.post<TRes>('https://bfc-anonymous.openblock.vip/rpc', {
		jsonrpc: '2.0',
		method,
		params,
		id: 4,
	});
};

export const getAnonymousRestoreValue = (params: {
	value1: number[];
	value2: number[];
	signature: number[];
	objectid: string;
	publickey: number[];
}) => {
	return anonymousRequest<
		{
			jsonrpc: '2.0';
			id: number;
			result: {
				result1: number;
				result2: number;
				operation: 'anonymous_restore_value';
				timestamp: number;
			};
			error: null;
		},
		typeof params
	>({
		method: 'bfcx_getAnonymousRestoreValue',
		params,
	});
};
