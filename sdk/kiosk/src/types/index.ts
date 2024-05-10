// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SharedObjectRef } from '@benfen/bfc.js/bcs';
import { SuiObjectRef } from '@benfen/bfc.js/client';
import { TransactionArgument } from '@benfen/bfc.js/transactions';

export * from './kiosk';
export * from './transfer-policy';
export * from './env';

/**
 * A valid argument for any of the Kiosk functions.
 */
export type ObjectArgument = string | TransactionArgument | SharedObjectRef | SuiObjectRef;
