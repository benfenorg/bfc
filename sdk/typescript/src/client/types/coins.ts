// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

export type CoinBalance = {
	coinType: string;
	coinObjectCount: number;
	totalBalance: string;
	lockedBalance: Record<string, string>;
};
