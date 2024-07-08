// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { useEffect, useState } from 'react';

const readLockDurationMs = 1000 * 60 * 60 * 6; // read only unlocked for 6 hours after unlocking

export function useIsAccountReadLocked(account: SerializedUIAccount | null) {
	const [now, setNow] = useState(Date.now());
	useEffect(() => {
		const interval = setInterval(() => {
			setNow(Date.now());
		}, 1000);
		return () => clearInterval(interval);
	}, []);
	return !account?.lastUnlockedOn || now - account.lastUnlockedOn > readLockDurationMs;
}
