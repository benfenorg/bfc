// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/**
 *  ######################################
 *  ### DO NOT EDIT THIS FILE DIRECTLY ###
 *  ######################################
 *
 * This file is generated from:
 * /crates/bfc-open-rpc/spec/openrpc.json
 */

import type { GetLatestSuiSystemStateParams } from '@benfen/bfc.js/client';
import type { UseSuiClientQueryOptions } from '../useSuiClientQuery.js';
import { useSuiClientQuery } from '../useSuiClientQuery.js';

export function useLatestSuiSystemState(
	params: GetLatestSuiSystemStateParams = {},
	options?: UseSuiClientQueryOptions<'getLatestSuiSystemState'>,
) {
	return useSuiClientQuery(
		{
			method: 'getLatestSuiSystemState',
			params,
		},
		options,
	);
}
