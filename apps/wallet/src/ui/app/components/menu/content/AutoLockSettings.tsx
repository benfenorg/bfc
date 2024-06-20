// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_src/ui/app/shared/text';

import { useNextMenuUrl } from '../hooks';
import AutoLockTimerSelector from './AutoLockTimerSelector';
import { MenuLayout } from './MenuLayout';

export function AutoLockSettings() {
	const backUrl = useNextMenuUrl(true, '/');
	return (
		<MenuLayout title="Auto Lock" back={backUrl}>
			<div className="flex flex-col gap-1.25">
				<Text color="bfc-text1" weight="normal" variant="bodySmall">
					Set the idle time in minutes before BenFen Wallet locks itself.
				</Text>
				<AutoLockTimerSelector />
			</div>
		</MenuLayout>
	);
}
