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
                    className="flex items-center justify-center h-34 xl:h-36 text-white text-2xl font-bold"
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
