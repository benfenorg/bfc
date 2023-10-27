// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useEffect } from 'react';
import { PersistableStorage } from '../utils/persistableStorage';

export const ANALYTICS_COOKIE_CATEGORY = 'analytics';
export const NECESSARY_COOKIE_CATEGORY = 'necessary';

type CookieConsentConfig = { onBeforeLoad: () => void };

export function useCookieConsentBanner<T>(
	storageInstance: PersistableStorage<T>,
	options: CookieConsentConfig,
) {
	useEffect(() => {
		storageInstance.persist();
	}, [storageInstance]);
}
