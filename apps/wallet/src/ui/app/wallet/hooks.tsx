// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useAppSelector } from '_hooks';
import { useEffect } from 'react';
import { useLocation, useNavigate, useSearchParams } from 'react-router-dom';

export function useLockedGuard(requiredLockedStatus: boolean) {
	const navigate = useNavigate();
	const [searchParams] = useSearchParams();
	const { pathname, search, state } = useLocation();
	const { isInitialized, isLocked } = useAppSelector(
		({ account: { isInitialized, isLocked } }) => ({
			isInitialized,
			isLocked,
		}),
	);
	const loading = isInitialized === null || isLocked === null;
	const guardAct = !loading && isInitialized && requiredLockedStatus !== isLocked;
	const nextUrl = searchParams.get('url') || '/';
	useEffect(() => {
		if (guardAct) {
			navigate(
				requiredLockedStatus ? nextUrl : `/locked?url=${encodeURIComponent(pathname + search)}`,
				{ replace: true, state },
			);
		}
	}, [guardAct, navigate, requiredLockedStatus, pathname, search, state, nextUrl]);

	return loading || guardAct;
}
