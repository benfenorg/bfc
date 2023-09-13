// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMemo, useState } from 'react';
import { ArrowLeft12 } from '@mysten/icons';
import { EpochsActivityTable } from './EpochsActivityTable';
import { TransactionsActivityTable } from './TransactionsActivityTable';
import { CheckpointsTable } from '../checkpoints/CheckpointsTable';
import { Tabs, TabsContent } from '~/ui/Tabs';
import { TopValidatorsCard } from '../../components/top-validators-card/TopValidatorsCard';
import { useNavigate } from "react-router-dom";

const VALID_TABS = ['transactions', 'epochs', 'checkpoints'];

type Props = {
	initialTab?: string | null;
	initialLimit: number;
	disablePagination?: boolean;
};

// const AUTO_REFRESH_ID = 'auto-refresh';
const REFETCH_INTERVAL_SECONDS = 10;
const REFETCH_INTERVAL = REFETCH_INTERVAL_SECONDS * 1000;

export function RecentActivity({ initialTab, initialLimit, disablePagination }: Props) {
	const [paused] = useState(false);

	const activeTab = useMemo(()=>{
		if(initialTab && VALID_TABS.includes(initialTab)){
			return initialTab
		}
		return 'transactions'
	},[initialTab])

	const navigate = useNavigate();
	const refetchInterval = paused ? undefined : REFETCH_INTERVAL;

	const pageBack = () => {
		navigate(-1)
	}
	return (
		<div>
			<Tabs size="md" value={activeTab}>
				<div className="flex items-center relative cursor-pointer" onClick={pageBack}>
					<ArrowLeft12 width={20} height={20}/><span className="capitalize ml-2 text-base font-semibold">{activeTab}</span>
				</div>
				<TabsContent value="transactions">
					<TransactionsActivityTable
						refetchInterval={refetchInterval}
						initialLimit={initialLimit}
						disablePagination={disablePagination}
						transactionKindFilter={undefined}
					/>
				</TabsContent>
				<TabsContent value="epochs">
					<EpochsActivityTable
						refetchInterval={refetchInterval}
						initialLimit={initialLimit}
						disablePagination={disablePagination}
					/>
				</TabsContent>
				<TabsContent value="checkpoints">
					<CheckpointsTable
						refetchInterval={refetchInterval}
						initialLimit={initialLimit}
						disablePagination={disablePagination}
					/>
				</TabsContent>
				<TabsContent value="validators">
					<TopValidatorsCard limit={initialLimit} showIcon />
				</TabsContent>
			</Tabs>
		</div>
	);
}
