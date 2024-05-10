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

import type { GetNormalizedMoveModuleParams } from '../../../client/index.js';
import type { UseSuiClientQueryOptions } from '../useSuiClientQuery.js';
import { useSuiClientQuery } from '../useSuiClientQuery.js';

export function useNormalizedMoveModule(
	params: GetNormalizedMoveModuleParams,
	options?: UseSuiClientQueryOptions<'getNormalizedMoveModule'>,
) {
	return useSuiClientQuery(
		{
			method: 'getNormalizedMoveModule',
			params,
		},
		options,
	);
}
