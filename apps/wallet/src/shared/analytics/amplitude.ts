// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type UserSession } from '@amplitude/analytics-types';
import { PersistableStorage } from '@mysten/core';

export const persistableStorage = new PersistableStorage<UserSession>();

export async function initAmplitude() {}

export function getUrlWithDeviceId(url: URL) {
	return url;
}
