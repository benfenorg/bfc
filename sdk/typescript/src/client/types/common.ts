// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import type { CallArg } from '../../bcs/index.js';

export type BenfenJsonValue = boolean | number | string | CallArg | Array<BenfenJsonValue>;
export type Order = 'ascending' | 'descending';
export type Unsubscribe = () => Promise<boolean>;
