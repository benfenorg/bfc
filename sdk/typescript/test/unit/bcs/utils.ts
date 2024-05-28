// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD:sdk/typescript/test/unit/bcs/utils.ts
import type { BCS } from '../../../src/bcs/src/index.js';
=======
import type { BCS } from '../src/index.js';
>>>>>>> mainnet-v1.24.1:sdk/bcs/tests/utils.ts

/** Serialize and deserialize the result. */
export function serde(bcs: BCS, type: any, data: any): any {
	let ser = bcs.ser(type, data).toString('hex');
	let de = bcs.de(type, ser, 'hex');
	return de;
}
