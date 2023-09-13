// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { RecentActivity } from '../../components/Activity/RecentActivity';
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useSearchParamsMerged } from '~/ui/utils/LinkWithQuery';

const TRANSACTIONS_LIMIT = 20;

export function Recent() {
	const [searchParams] = useSearchParamsMerged();
	const tab = searchParams.get('tab')
	return (
		<PageLayout
			content={
				<div data-testid="transaction-page" id="transaction" className="mx-auto">
					<ErrorBoundary>
						<RecentActivity initialLimit={TRANSACTIONS_LIMIT} initialTab={tab} />
					</ErrorBoundary>
				</div>
			}
		/>
	);
}
