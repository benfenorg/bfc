// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { type EpochPage } from '@mysten/sui.js';
=======
import { type EpochPage } from '@mysten/sui.js/client';
>>>>>>> heads/mainnet-v1.9.1
import { Text } from '@mysten/ui';

import { SuiAmount } from '../Table/SuiAmount';
import { TxTimeType } from '../tx-time/TxTimeType';
<<<<<<< HEAD
=======
import { HighlightedTableCol } from '~/components/Table/HighlightedTableCol';
>>>>>>> heads/mainnet-v1.9.1
import { CheckpointSequenceLink, EpochLink } from '~/ui/InternalLink';
import { getEpochStorageFundFlow } from '~/utils/getStorageFundFlow';

// Generate table data from the epochs data
export const genTableDataFromEpochsData = (results: EpochPage) => ({
	data: results?.data.map((epoch) => ({
<<<<<<< HEAD
		epoch: <EpochLink epoch={epoch.epoch.toString()} />,
=======
		epoch: (
			<HighlightedTableCol first>
				<EpochLink epoch={epoch.epoch.toString()} />
			</HighlightedTableCol>
		),
>>>>>>> heads/mainnet-v1.9.1
		transactions: <Text variant="bodySmall/medium">{epoch.epochTotalTransactions}</Text>,
		stakeRewards: <SuiAmount amount={epoch.endOfEpochInfo?.totalStakeRewardsDistributed} />,
		checkpointSet: (
			<div>
				<CheckpointSequenceLink sequence={epoch.firstCheckpointId.toString()} />
				{` - `}
				<CheckpointSequenceLink
					sequence={epoch.endOfEpochInfo?.lastCheckpointId.toString() ?? ''}
				/>
			</div>
		),
		storageNetInflow: (
			<div className="pl-3">
				<SuiAmount amount={getEpochStorageFundFlow(epoch.endOfEpochInfo).netInflow} />
			</div>
		),
		time: <TxTimeType timestamp={Number(epoch.endOfEpochInfo?.epochEndTimestamp ?? 0)} />,
	})),
	columns: [
		{
			header: 'Epoch',
			accessorKey: 'epoch',
		},
		{
			header: 'Transaction Blocks',
			accessorKey: 'transactions',
		},
		{
			header: 'Stake Rewards',
			accessorKey: 'stakeRewards',
		},
		{
			header: 'Checkpoint Set',
			accessorKey: 'checkpointSet',
		},
		{
			header: 'Storage Net Inflow',
			accessorKey: 'storageNetInflow',
		},
		{
			header: 'Epoch End',
			accessorKey: 'time',
		},
	],
});
