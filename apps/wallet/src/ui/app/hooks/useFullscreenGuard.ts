// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useEffect } from 'react';
=======
import { AppType } from '_redux/slices/app/AppType';
import { openInNewTab } from '_shared/utils';
import { useEffect, useRef } from 'react';
>>>>>>> mainnet-v1.24.1

import useAppSelector from './useAppSelector';

export default function useFullscreenGuard(enabled: boolean) {
	const appType = useAppSelector((state) => state.app.appType);
	useEffect(() => {
		if (enabled && appType === AppType.popup) {
			openInNewTab().finally(() => window.close());
		}
	}, [appType, enabled]);
	return !enabled && appType === AppType.unknown;
}
