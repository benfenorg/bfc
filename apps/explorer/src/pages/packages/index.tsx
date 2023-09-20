// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { PageLayout } from '~/components/Layout/PageLayout';
import { TopPackagesCard } from '~/components/top-packages/TopPackagesCard';

function Home() {
	return (
		<PageLayout
			gradientContent={
				<div
					data-testid="home-page"
					className="h-34 flex items-center justify-center text-2xl font-bold text-white xl:h-36"
				>
					Packages
				</div>
			}
			content={
				<div data-testid="packages-page" id="home-content" className="mx-auto">
					<ErrorBoundary>
						<TopPackagesCard />
					</ErrorBoundary>
				</div>
			}
		/>
	);
}

export default Home;
