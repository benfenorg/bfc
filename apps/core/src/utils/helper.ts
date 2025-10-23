// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { hex2BfcAddress } from '@benfen/bfc.js/src/utils';

export const combineAusdType = (pkg: string) => {
	return `${hex2BfcAddress(pkg)}::anonymous_usd::ANONYMOUS_USD`;
};
