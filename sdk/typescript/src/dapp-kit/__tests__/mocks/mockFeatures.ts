// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { vi } from 'vitest';

import type {
	IdentifierRecord,
	SuiFeatures,
	SuiSignMessageFeature,
} from '../../../wallet-standard/index.js';

export const signMessageFeature: SuiSignMessageFeature = {
	'bfc:signMessage': {
		version: '1.0.0',
		signMessage: vi.fn(),
	},
};

export const superCoolFeature: IdentifierRecord<unknown> = {
	'my-dapp:super-cool-feature': {
		version: '1.0.0',
		superCoolFeature: vi.fn(),
	},
};

export const suiFeatures: SuiFeatures = {
	...signMessageFeature,
	'bfc:signPersonalMessage': {
		version: '1.0.0',
		signPersonalMessage: vi.fn(),
	},
	'bfc:signTransactionBlock': {
		version: '1.0.0',
		signTransactionBlock: vi.fn(),
	},
	'bfc:signAndExecuteTransactionBlock': {
		version: '1.0.0',
		signAndExecuteTransactionBlock: vi.fn(),
	},
};
