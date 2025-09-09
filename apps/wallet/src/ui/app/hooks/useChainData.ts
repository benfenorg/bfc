// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useRpcData } from '@mysten/core';

import useAppSelector from './useAppSelector';

export const useChainData = () => {
	const [_, activeRpcUrl] = useAppSelector(({ app }) => [app.apiEnv, app.customRPC]);

	return useRpcData(activeRpcUrl as string);
};
