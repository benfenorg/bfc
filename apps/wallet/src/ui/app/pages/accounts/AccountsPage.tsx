// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Outlet } from 'react-router-dom';
<<<<<<< HEAD
=======

>>>>>>> mainnet-v1.24.1
import { Toaster } from '../../shared/toaster';

export function AccountsPage() {
	return (
<<<<<<< HEAD
		<PageLayout>
			<Outlet />
			<Toaster bottomNavEnabled={false} />
		</PageLayout>
=======
		<>
			<Outlet />
			<Toaster />
		</>
>>>>>>> mainnet-v1.24.1
	);
}
