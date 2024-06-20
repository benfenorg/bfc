// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import Loading from '_components/loading';
import { useInitializedGuard } from '_hooks';
import PageLayout from '_pages/layout';
import { Outlet, useLocation } from 'react-router-dom';

import st from './InitializePage.module.scss';

const InitializePage = () => {
	const { pathname } = useLocation();
	const checkingInitialized = useInitializedGuard(
		/^\/initialize\/backup(-imported)?(\/)?$/.test(pathname),
	);
	return (
		<PageLayout forceFullscreen={true} className={st.container}>
			<Loading loading={checkingInitialized}>
				<Outlet />
			</Loading>
		</PageLayout>
	);
};

export default InitializePage;
