// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { lazy } from 'react';

import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { Activity } from '~/components/Activity';
import { Overview } from '~/components/HomeMetrics';
import { PageLayout } from '~/components/Layout/PageLayout';
import { TransactionsCardGraph } from '~/components/TransactionsCardGraph';
import { useNetwork } from '~/context';
import { Network } from '~/utils/api/DefaultRpcClient';

const ValidatorMap = lazy(() => import('../../components/validator-map'));

const TRANSACTIONS_LIMIT = 25;

function Home() {
	const [network] = useNetwork();
	const isSuiTokenCardEnabled = network === Network.MAINNET;
	return (
		<PageLayout
			gradientContent={
				<div data-testid="home-page" className="grid gap-4 sm:grid-flow-col">
					<div className="col-span">
						<Overview />
					</div>
					
					<div className="col-span sm:col-span-2">
						<TransactionsCardGraph />
					</div>
				</div>
			}
			content={
				<div className="home-page-grid-container-bottom" id="home-content">
					<div style={{ gridArea: 'activity' }}>
						<ErrorBoundary>
							<Activity initialLimit={TRANSACTIONS_LIMIT} disablePagination />
						</ErrorBoundary>
					</div>
					{/* <div style={{ gridArea: 'packages' }}>
						<TopPackagesCard />
					</div> */}
				</div>
			}
		/>
	);
}

export default Home;
