// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SUI_TYPE_ARG, type SuiAddress, type StakeObject } from '@benfen/bfc.js';
import { useFormatCoin, useGetTimeBeforeEpochNumber } from '@mysten/core';
import { cx, cva, type VariantProps } from 'class-variance-authority';
import { Link } from 'react-router-dom';

import { ValidatorLogo } from '../validators/ValidatorLogo';
import { NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE } from '_src/shared/constants';
import { CountDownTimer } from '_src/ui/app/shared/countdown-timer';
import { Text } from '_src/ui/app/shared/text';

=======
import { NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE } from '_src/shared/constants';
import { CountDownTimer } from '_src/ui/app/shared/countdown-timer';
import { Text } from '_src/ui/app/shared/text';
import { IconTooltip } from '_src/ui/app/shared/tooltip';
import { useFormatCoin, useGetTimeBeforeEpochNumber } from '@mysten/core';
import { type StakeObject } from '@mysten/sui.js/client';
import { SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import { cva, cx, type VariantProps } from 'class-variance-authority';
>>>>>>> mainnet-v1.24.1
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';

import { ValidatorLogo } from '../validators/ValidatorLogo';

export enum StakeState {
	WARM_UP = 'WARM_UP',
	EARNING = 'EARNING',
	COOL_DOWN = 'COOL_DOWN',
	WITHDRAW = 'WITHDRAW',
	IN_ACTIVE = 'IN_ACTIVE',
}

const STATUS_COPY = {
	[StakeState.WARM_UP]: 'Starts Earning',
	[StakeState.EARNING]: 'Staking Rewards',
	[StakeState.COOL_DOWN]: 'Available to withdraw',
	[StakeState.WITHDRAW]: 'Withdraw',
	[StakeState.IN_ACTIVE]: 'Inactive',
};

const STATUS_VARIANT = {
	[StakeState.WARM_UP]: 'warmUp',
	[StakeState.EARNING]: 'earning',
	[StakeState.COOL_DOWN]: 'coolDown',
	[StakeState.WITHDRAW]: 'withDraw',
	[StakeState.IN_ACTIVE]: 'inActive',
} as const;
interface DelegationObjectWithValidator extends StakeObject {
	validatorAddress: SuiAddress;
}

const cardStyle = cva(
	[
		'group flex no-underline flex-col gap-2.5 py-4 px-2.5 box-border w-full rounded-2xl border border-solid',
	],
	{
		variants: {
			variant: {
				warmUp: '',
				earning: '',
				coolDown: '',
				withDraw: '',
				inActive: '',
			},
		},
	},
);

export interface StakeCardContentProps extends VariantProps<typeof cardStyle> {
	statusLabel: string;
	statusText: string;
	children?: ReactNode;
	earnColor?: boolean;
	earningRewardEpoch?: number | null;
}

function StakeCardContent({
	children,
	statusLabel,
	statusText,
	variant,
	earnColor,
	earningRewardEpoch,
}: StakeCardContentProps) {
	const { data: rewardEpochTime } = useGetTimeBeforeEpochNumber(earningRewardEpoch || 0);
	return (
		<div className={cardStyle({ variant })}>
			{children}
			<div className="flex flex-col">
				<div className="text-body/[18px] font-normal text-bfc-text3">{statusLabel}</div>
				<div className={cx('text-body/[18px] font-normal text-bfc-text3')}>
					{earningRewardEpoch && rewardEpochTime > 0 ? (
						<CountDownTimer
							timestamp={rewardEpochTime}
							variant="body"
							label="in"
							color="bfc-text3"
							weight="normal"
						/>
					) : (
						statusText
					)}
				</div>
			</div>
		</div>
	);
}

interface StakeCardProps {
	delegationObject: DelegationObjectWithValidator;
	currentEpoch: number;
	inactiveValidator?: boolean;
}

// For delegationsRequestEpoch n  through n + 2, show Start Earning
// Show epoch number or date/time for n + 3 epochs
export function StakeCard({
	delegationObject,
	currentEpoch,
	inactiveValidator = false,
}: StakeCardProps) {
	const { stakedSuiId, principal, stakeRequestEpoch, estimatedReward, validatorAddress } =
		delegationObject;

	// TODO: Once two step withdraw is available, add cool down and withdraw now logic
	// For cool down epoch, show Available to withdraw add rewards to principal
	// Reward earning epoch is 2 epochs after stake request epoch
	const earningRewardsEpoch =
		Number(stakeRequestEpoch) + NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE;
	const isEarnedRewards = currentEpoch >= Number(earningRewardsEpoch);
	const delegationState = inactiveValidator
		? StakeState.IN_ACTIVE
		: isEarnedRewards
		? StakeState.EARNING
		: StakeState.WARM_UP;

	const rewards = isEarnedRewards && estimatedReward ? BigInt(estimatedReward) : 0n;

	// For inactive validator, show principal + rewards
	const [principalStaked, symbol] = useFormatCoin(
		inactiveValidator ? principal + rewards : principal,
		SUI_TYPE_ARG,
	);
	const [rewardsStaked] = useFormatCoin(rewards, SUI_TYPE_ARG);
	const isEarning = delegationState === StakeState.EARNING && rewards > 0n;

	// Applicable only for warm up
	const epochBeforeRewards = delegationState === StakeState.WARM_UP ? earningRewardsEpoch : null;

	const statusText = {
		// Epoch time before earning
		[StakeState.WARM_UP]: `Epoch #${earningRewardsEpoch}`,
		[StakeState.EARNING]: `${rewardsStaked} ${symbol}`,
		// Epoch time before redrawing
		[StakeState.COOL_DOWN]: `Epoch #`,
		[StakeState.WITHDRAW]: 'Now',
		[StakeState.IN_ACTIVE]: 'Not earning rewards',
	};

	return (
		<Link
			data-testid="stake-card"
			to={`/stake/delegation-detail?${new URLSearchParams({
				validator: validatorAddress,
				staked: stakedSuiId,
			}).toString()}`}
			className="no-underline active:text-bfc-border hover:text-bfc-border visited:text-bfc-border"
		>
			<StakeCardContent
				variant={STATUS_VARIANT[delegationState]}
				statusLabel={STATUS_COPY[delegationState]}
				statusText={statusText[delegationState]}
				earnColor={isEarning}
				earningRewardEpoch={Number(epochBeforeRewards)}
			>
				<div className="flex">
					<ValidatorLogo
						validatorAddress={validatorAddress}
						size="subtitle"
						iconSize="md"
						stacked
						activeEpoch={delegationObject.stakeRequestEpoch}
					/>
				</div>
				<div className="flex-1 flex items-baseline gap-1.25">
					<Text variant="body" weight="medium" color="bfc-text1">
						{principalStaked}
					</Text>
					<Text variant="body" weight="medium" color="bfc-text2">
						{symbol}
					</Text>
				</div>
			</StakeCardContent>
		</Link>
	);
}
