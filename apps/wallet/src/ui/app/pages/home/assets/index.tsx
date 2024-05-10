// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useUnlockedGuard } from '_src/ui/app/hooks/useUnlockedGuard';
import { Route, Routes } from 'react-router-dom';

import { HiddenAssetsPage, NftsPage } from '..';

function AssetsPage() {
	if (useUnlockedGuard()) {
		return null;
	}
	return (
		<Routes>
			<Route path="/hidden-assets" element={<HiddenAssetsPage />} />
			<Route path="/*" element={<NftsPage />} />
		</Routes>
	);
}

export default AssetsPage;
