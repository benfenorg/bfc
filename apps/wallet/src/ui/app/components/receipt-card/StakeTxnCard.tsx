// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { StakeAmount } from '_app/staking/home/StakeAmount';
import { ValidatorLogo } from '_app/staking/validators/ValidatorLogo';
import {
	NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE,
	NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_STARTS,
} from '_src/shared/constants';
import { CountDownTimer } from '_src/ui/app/shared/countdown-timer';
import { Text } from '_src/ui/app/shared/text';
import { IconTooltip } from '_src/ui/app/shared/tooltip';
import type { SuiEvent } from '@benfen/bfc.js/client';
import {
	formatPercentageDisplay,
	useGetTimeBeforeEpochNumber,
	useGetValidatorsApy,
} from '@mysten/core';

type StakeTxnCardProps = {
	event: SuiEvent;
};

// For Staked Transaction use moveEvent Field to get the validator address, delegation amount, epoch
export function StakeTxnCard({ event }: StakeTxnCardProps) {
	const json = event.parsedJson as { amount: string; validator_address: string; epoch: string };
	const validatorAddress = json?.validator_address;
	const stakedAmount = json?.amount;
	const stakedEpoch = Number(json?.epoch || '0');

	const { data: rollingAverageApys } = useGetValidatorsApy();

	const { apy, isApyApproxZero } = rollingAverageApys?.[validatorAddress] ?? {
		apy: null,
	};
	// Reward will be available after 2 epochs
	// TODO: Get epochStartTimestampMs/StartDate
	// for staking epoch + NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE
	const startEarningRewardsEpoch = Number(stakedEpoch) + NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_STARTS;

	const redeemableRewardsEpoch =
		Number(stakedEpoch) + NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE;

	const { data: timeBeforeStakeRewardsStarts } =
		useGetTimeBeforeEpochNumber(startEarningRewardsEpoch);

	const { data: timeBeforeStakeRewardsRedeemable } =
		useGetTimeBeforeEpochNumber(redeemableRewardsEpoch);

	return (
		<div className="flex flex-col divide-y divide-solid divide-bfc-border divide-x-0 border border-solid border-bfc-border rounded-lg p-2.5">
			{validatorAddress && (
				<div className="mb-2.5 w-full">
					<ValidatorLogo
						validatorAddress={validatorAddress}
						showAddress
						iconSize="md"
						size="body"
						activeEpoch={json?.epoch}
					/>
				</div>
			)}
			{stakedAmount && (
				<div className="h-10 flex justify-between w-full items-center">
					<Text variant="body" weight="normal" color="bfc-text1">
						Stake
					</Text>
					<StakeAmount balance={stakedAmount} variant="body" />
				</div>
			)}
			<div className="flex justify-between w-full py-2.5">
				<div className="flex gap-1 justify-center items-center text-bfc-text2">
					<Text variant="body" weight="normal" color="bfc-text2">
						APY
					</Text>
					<IconTooltip tip="This is the Annualized Percentage Yield of the a specific validator’s past operations. Note there is no guarantee this APY will be true in the future." />
				</div>
				<Text variant="body" weight="medium" color="bfc-text1">
					{formatPercentageDisplay(apy, '--', isApyApproxZero)}
				</Text>
			</div>
			<div className="flex flex-col">
				<div className="flex justify-between w-full py-2.5">
					<div className="flex gap-1 items-center text-steel">
						<Text variant="body" weight="medium" color="steel-darker">
							{timeBeforeStakeRewardsStarts > 0
								? 'Staking Rewards Start'
								: 'Staking Rewards Started'}
						</Text>
					</div>

					{timeBeforeStakeRewardsStarts > 0 ? (
						<CountDownTimer
							timestamp={timeBeforeStakeRewardsStarts}
							variant="body"
							color="bfc-text1"
							weight="normal"
							label="in"
							endLabel="--"
						/>
					) : (
						<Text variant="body" weight="normal" color="bfc-text1">
							Epoch #{startEarningRewardsEpoch}
						</Text>
					)}
				</div>
				<div className="py-2.5 flex justify-between w-full">
					<div className="flex gap-1 flex-1 items-center text-bfc">
						<Text variant="body" weight="medium" color="bfc-text1">
							Staking Rewards Redeemable
						</Text>
					</div>
					<div className="flex-1 flex justify-end gap-1 items-center">
						{timeBeforeStakeRewardsRedeemable > 0 ? (
							<CountDownTimer
								timestamp={timeBeforeStakeRewardsRedeemable}
								variant="body"
								color="bfc-text1"
								weight="normal"
								label="in"
								endLabel="--"
							/>
						) : (
							<Text variant="body" weight="normal" color="bfc-text1">
								Epoch #{redeemableRewardsEpoch}
							</Text>
						)}
					</div>
				</div>
			</div>
		</div>
	);
}
