// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import PageLayout from '_pages/layout';
import { Outlet } from 'react-router-dom';

import { Toaster } from '../../shared/toaster';

export function AccountsPage() {
	return (
		<PageLayout>
			<Outlet />
			<Toaster bottomNavEnabled={false} />
		</PageLayout>
	);
}
