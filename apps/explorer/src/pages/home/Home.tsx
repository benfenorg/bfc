// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { Activity } from '~/components/Activity';
import { CurrentEpoch, Overview } from '~/components/HomeMetrics';
import { PageLayout } from '~/components/Layout/PageLayout';
import { TransactionsCardGraph } from '~/components/TransactionsCardGraph';

const TRANSACTIONS_LIMIT = 25;

function Home() {
	return (
		<PageLayout
			gradient={{
				content: (
					<div data-testid="home-page" className="home-page-grid-container-top">
						<div style={{ gridArea: 'overview' }}>
							<Overview />
						</div>
						<div style={{ gridArea: 'epoch' }}>
							<CurrentEpoch />
						</div>
						<div style={{ gridArea: 'transactions' }}>
							<TransactionsCardGraph />
						</div>
					</div>
				),
				size: 'lg',
			}}
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
