// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { BenfenObjectResponse } from '@benfen/bfc.js/client';

export const hasDisplayData = (obj: BenfenObjectResponse) => !!obj.data?.display?.data;
