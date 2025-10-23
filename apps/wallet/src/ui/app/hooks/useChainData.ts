// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { ENV_TO_API } from '_src/shared/api-env';
import { useRpcData } from '@mysten/core';

import useAppSelector from './useAppSelector';

export const useChainData = () => {
	const [apiEnv, activeRpcUrl] = useAppSelector(({ app }) => [app.apiEnv, app.customRPC]);
	const rpcUrl = ENV_TO_API[apiEnv];

	return useRpcData(rpcUrl || (activeRpcUrl as string));
};
