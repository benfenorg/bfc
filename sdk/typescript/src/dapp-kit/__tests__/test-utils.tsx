// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { SuiClient } from '../../client/index.js';
import { SuiClientProvider } from '../index.js';

export function createSuiClientContextWrapper(client: SuiClient) {
	return function SuiClientContextWrapper({ children }: { children: React.ReactNode }) {
		return <SuiClientProvider networks={{ test: client }}>{children}</SuiClientProvider>;
	};
}
