// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';

import useAppSelector from './useAppSelector';
import { useRestrictedGuard } from './useRestrictedGuard';

export default function useInitializedGuard(initializedRequired: boolean, enabled = true) {
	const restricted = useRestrictedGuard();
<<<<<<< HEAD

	const isInitialized = useAppSelector(({ account }) => account.isInitialized);
	const loading = isInitialized === null;
	const navigate = useNavigate();
	const guardAct = useMemo(
		() => !restricted && !loading && initializedRequired !== isInitialized,
		[loading, initializedRequired, isInitialized, restricted],
	);
=======
	const { data: allAccounts, isPending } = useAccounts();
	const isInitialized = !!allAccounts?.length;
	const navigate = useNavigate();
	const guardAct = !restricted && !isPending && initializedRequired !== isInitialized && enabled;
>>>>>>> mainnet-v1.24.1
	useEffect(() => {
		if (guardAct) {
			navigate(isInitialized ? '/' : '/accounts/welcome', { replace: true });
		}
	}, [guardAct, isInitialized, navigate]);
<<<<<<< HEAD
	return loading || guardAct;
=======
	return isPending || guardAct;
>>>>>>> mainnet-v1.24.1
}
