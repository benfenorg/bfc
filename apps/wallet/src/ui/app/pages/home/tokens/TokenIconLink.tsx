// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SUI_TYPE_ARG } from '@benfen/bfc.js';
import { useFormatCoin } from '@mysten/core';
=======
import { LargeButton } from '_app/shared/LargeButton';
import { ampli } from '_src/shared/analytics/ampli';
import {
	DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	DELEGATED_STAKES_QUERY_STALE_TIME,
} from '_src/shared/constants';
import { Text } from '_src/ui/app/shared/text';
import { useFormatCoin, useGetDelegatedStake } from '@mysten/core';
>>>>>>> mainnet-v1.24.1
import { WalletActionStake24 } from '@mysten/icons';
import { useMemo } from 'react';

<<<<<<< HEAD
import { LargeButton } from '_app/shared/LargeButton';
import { DelegatedAPY } from '_app/shared/delegated-apy';
import { useGetDelegatedStake } from '_app/staking/useGetDelegatedStake';
import { ampli } from '_src/shared/analytics/ampli';

export function TokenIconLink({ accountAddress }: { accountAddress: string }) {
	const { data: delegatedStake, isLoading } = useGetDelegatedStake(accountAddress);
=======
export function TokenIconLink({
	accountAddress,
	disabled,
}: {
	accountAddress: string;
	disabled: boolean;
}) {
	const { data: delegatedStake, isPending } = useGetDelegatedStake({
		address: accountAddress,
		staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
		refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	});
>>>>>>> mainnet-v1.24.1

	// Total active stake for all delegations
	const totalActivePendingStake = useMemo(() => {
		if (!delegatedStake) return 0n;

		return delegatedStake.reduce(
			(acc, curr) => curr.stakes.reduce((total, { principal }) => total + BigInt(principal), acc),

			0n,
		);
	}, [delegatedStake]);

	const stakedValidators = delegatedStake?.map(({ validatorAddress }) => validatorAddress) || [];

	const [formatted, symbol, queryResult] = useFormatCoin(totalActivePendingStake, SUI_TYPE_ARG);

	return (
		<LargeButton
			to="/stake"
			onClick={() => {
				ampli.clickedStakeSui({
					isCurrentlyStaking: totalActivePendingStake > 0,
					sourceFlow: 'Home page',
				});
			}}
			loading={isPending || queryResult.isPending}
			before={<WalletActionStake24 />}
<<<<<<< HEAD
			center
			after={totalActivePendingStake ? <DelegatedAPY stakedValidators={stakedValidators} /> : null}
=======
			data-testid={`stake-button-${formatted}-${symbol}`}
>>>>>>> mainnet-v1.24.1
		>
			<div className="flex flex-col gap-1.25">
				<div>{totalActivePendingStake ? 'Currently Staked' : 'Stake and Earn SUI'}</div>
				{!!totalActivePendingStake && (
					<div>
						{formatted} {symbol}
					</div>
				)}
			</div>
		</LargeButton>
	);
}
