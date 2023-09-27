// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import { TopPackagesTable } from './TopPackagesTable';
import { ErrorBoundary } from '../error-boundary/ErrorBoundary';
import { useEnhancedRpcClient } from '~/hooks/useEnhancedRpc';
import { FilterList } from '~/ui/FilterList';

export type DateFilter = '3D' | '7D' | '30D';
export type ApiDateFilter = 'rank3Days' | 'rank7Days' | 'rank30Days';
export const FILTER_TO_API_FILTER: Record<DateFilter, ApiDateFilter> = {
	'3D': 'rank3Days',
	'7D': 'rank7Days',
	'30D': 'rank30Days',
};

export function TopPackagesCard() {
	const rpc = useEnhancedRpcClient();
	const [selectedFilter, setSelectedFilter] = useState<DateFilter>('3D');

	const { data, isLoading } = useQuery({
		queryKey: ['top-packages', selectedFilter],
		queryFn: async () => rpc.getMoveCallMetrics(),
	});

	const filteredData = data ? data[FILTER_TO_API_FILTER[selectedFilter]] : [];

	return (
		<div>
			<div className="">
				<FilterList
					lessSpacing
					options={['3D', '7D', '30D']}
					value={selectedFilter}
					onChange={(val) => setSelectedFilter(val)}
				/>
			</div>
			<div className="obc-table-container mt-5">
				<ErrorBoundary>
					<TopPackagesTable data={filteredData} isLoading={isLoading} />
				</ErrorBoundary>
			</div>
		</div>
	);
}
