// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Card } from '_app/shared/card';
import Alert from '_components/alert';
import LoadingIndicator from '_components/loading/LoadingIndicator';
import { Text } from '_src/ui/app/shared/text';
import { IconTooltip } from '_src/ui/app/shared/tooltip';
import {
	calculateStakeShare,
	formatPercentageDisplay,
	useGetSystemState,
	useGetValidatorsApy,
} from '@mysten/core';
import { useMemo } from 'react';
import { useSearchParams } from 'react-router-dom';

import { useActiveAddress } from '../../hooks/useActiveAddress';
import { getStakeSuiBySuiId } from '../getStakeSuiBySuiId';
import { getTokenStakeSuiForValidator } from '../getTokenStakeSuiForValidator';
import { StakeAmount } from '../home/StakeAmount';
import { useGetDelegatedStake } from '../useGetDelegatedStake';
import { ValidatorLogo } from '../validators/ValidatorLogo';

type ValidatorFormDetailProps = {
	validatorAddress: string;
	unstake?: boolean;
};

export function ValidatorFormDetail({ validatorAddress, unstake }: ValidatorFormDetailProps) {
	const accountAddress = useActiveAddress();

	const [searchParams] = useSearchParams();
	const stakeIdParams = searchParams.get('staked');
	const {
		data: system,
		isLoading: loadingValidators,
		isError: errorValidators,
	} = useGetSystemState();

	const { data: stakeData, isLoading, isError, error } = useGetDelegatedStake(accountAddress || '');

	const { data: rollingAverageApys } = useGetValidatorsApy();

	const validatorData = useMemo(() => {
		if (!system) return null;
		return system.activeValidators.find((av) => av.suiAddress === validatorAddress);
	}, [validatorAddress, system]);

	//TODO: verify this is the correct validator stake balance
	const totalValidatorStake = validatorData?.stakingPoolSuiBalance || 0;

	const totalStake = useMemo(() => {
		if (!stakeData) return 0n;
		return unstake
			? getStakeSuiBySuiId(stakeData, stakeIdParams)
			: getTokenStakeSuiForValidator(stakeData, validatorAddress);
	}, [stakeData, stakeIdParams, unstake, validatorAddress]);

	const totalValidatorsStake = useMemo(() => {
		if (!system) return 0;
		return system.activeValidators.reduce(
			(acc, curr) => (acc += BigInt(curr.stakingPoolSuiBalance)),
			0n,
		);
	}, [system]);

	const totalStakePercentage = useMemo(() => {
		if (!system || !validatorData) return null;

		return calculateStakeShare(
			BigInt(validatorData.stakingPoolSuiBalance),
			BigInt(totalValidatorsStake),
		);
	}, [system, totalValidatorsStake, validatorData]);

	const { apy, isApyApproxZero } = rollingAverageApys?.[validatorAddress] ?? {
		apy: null,
	};

	if (isLoading || loadingValidators) {
		return (
			<div className="p-2 w-full flex justify-center items-center h-full">
				<LoadingIndicator />
			</div>
		);
	}

	if (isError || errorValidators) {
		return (
			<div className="p-2">
				<Alert>
					<div className="mb-1 font-semibold">
						{error?.message ?? 'Error loading validator data'}
					</div>
				</Alert>
			</div>
		);
	}

	return (
		<div className="w-full">
			{validatorData && (
				<Card
					header={
						<div className="h-10 flex gap-1.25 items-center justify-center">
							<ValidatorLogo
								validatorAddress={validatorAddress}
								iconSize="sm"
								size="body"
								isTitle
							/>
						</div>
					}
					footer={
						!unstake && (
							<div className="pb-2.5 px-2.5 w-full flex items-center justify-between">
								<Text variant="body" weight="normal" color="bfc-text2">
									Your Staked BFC
								</Text>

								<StakeAmount balance={totalStake} variant="body" />
							</div>
						)
					}
				>
					<div className="divide-x flex divide-solid divide-bfc-border divide-y-0 flex-col gap-2.5 mx-2.5 py-2.5">
						<div className="flex items-center justify-between">
							<div className="flex items-center gap-1.25 grow">
								<Text variant="body" weight="normal" color="bfc-text2">
									Staking APY
								</Text>
								<IconTooltip tip="This is the Annualized Percentage Yield of the a specific validator’s past operations. Note there is no guarantee this APY will be true in the future." />
							</div>

							<Text variant="body" weight="medium" color="bfc-text1">
								{formatPercentageDisplay(apy, '--', isApyApproxZero)}
							</Text>
						</div>
						<div className="flex items-center justify-between">
							<div className="flex gap-1.25 items-center">
								<Text variant="body" weight="normal" color="bfc-text2">
									Stake Share
								</Text>
								<IconTooltip tip="The percentage of total stake managed by this validator" />
							</div>

							<Text variant="body" weight="medium" color="bfc-text1">
								{formatPercentageDisplay(totalStakePercentage)}
							</Text>
						</div>

						{!unstake && (
							<div className="flex items-center justify-between">
								<div className="flex gap-1.25 items-center">
									<Text variant="body" weight="normal" color="bfc-text2">
										Total Staked
									</Text>
									<IconTooltip tip="The total BFC staked on the network by this validator and its delegators, to validate the network and earn rewards." />
								</div>
								<StakeAmount balance={totalValidatorStake} variant="body" />
							</div>
						)}
					</div>
				</Card>
			)}
		</div>
	);
}
