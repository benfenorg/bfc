// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Card } from '_app/shared/card';
import { Text } from '_app/shared/text';
import NumberInput from '_components/number-input';
import {
	NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE,
	NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_STARTS,
} from '_src/shared/constants';
import { CountDownTimer } from '_src/ui/app/shared/countdown-timer';
import { useCoinMetadata, useFormatCoin, useGetTimeBeforeEpochNumber } from '@mysten/core';
import { Field, Form, useFormikContext } from 'formik';
import { memo, useMemo } from 'react';

import { parseAmount } from '../../helpers';
import { useActiveAddress, useTransactionGasBudget } from '../../hooks';
import { type FormValues } from './StakingCard';
import { createStakeTransaction } from './utils/transaction';

export type StakeFromProps = {
	validatorAddress: string;
	coinBalance: bigint;
	coinType: string;
	epoch?: string | number;
};

function StakeForm({ validatorAddress, coinBalance, coinType, epoch }: StakeFromProps) {
	const { values } = useFormikContext<FormValues>();

	const { data: metadata } = useCoinMetadata(coinType);
	const decimals = metadata?.decimals ?? 0;
	const [maxToken, symbol] = useFormatCoin(coinBalance, coinType);

	const transaction = useMemo(() => {
		if (!values.amount || !decimals) return null;
		const amountWithoutDecimals = parseAmount(values.amount, decimals);
		return createStakeTransaction(amountWithoutDecimals, validatorAddress);
	}, [values.amount, validatorAddress, decimals]);

	const activeAddress = useActiveAddress();
	const { data: gasBudget } = useTransactionGasBudget(activeAddress, transaction);

	// Reward will be available after 2 epochs
	const startEarningRewardsEpoch = Number(epoch || 0) + NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_STARTS;

	const redeemableRewardsEpoch =
		Number(epoch || 0) + NUM_OF_EPOCH_BEFORE_STAKING_REWARDS_REDEEMABLE;

	const { data: timeBeforeStakeRewardsStarts } =
		useGetTimeBeforeEpochNumber(startEarningRewardsEpoch);

	const { data: timeBeforeStakeRewardsRedeemable } =
		useGetTimeBeforeEpochNumber(redeemableRewardsEpoch);

	return (
		<Form className="flex flex-1 flex-col flex-nowrap items-center" autoComplete="off">
			<div className="flex flex-col justify-between items-center mb-5 w-full">
				<Text variant="body" color="bfc-text1" weight="medium">
					Enter the amount of BFC to stake
				</Text>
				<Text variant="body" color="bfc-text2" weight="normal">
					Available - {maxToken} {symbol}
				</Text>
			</div>
			<Card variant="gray">
				<div className="p-2.5 w-full flex bg-white">
					<Field
						data-testid="stake-amount-input"
						component={NumberInput}
						allowNegative={false}
						name="amount"
						className="w-full border-none text-bfc text-heading4/[22px] font-semibold bg-white placeholder:text-bfc-text2"
						decimals
						suffix={` ${symbol}`}
						autoFocus
					/>
				</div>
				<div className="p-2.5 flex justify-between w-full">
					<Text variant="body" weight="normal" color="bfc-text1">
						Staking Rewards Start
					</Text>
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
							{epoch ? `Epoch #${Number(startEarningRewardsEpoch)}` : '--'}
						</Text>
					)}
				</div>
				<div className="p-2.5 flex justify-between item-center w-full">
					<div className="flex-1">
						<Text variant="body" weight="normal" color="bfc-text1">
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
								{epoch ? `Epoch #${Number(redeemableRewardsEpoch)}` : '--'}
							</Text>
						)}
					</div>
				</div>
				<div className="px-2.5 w-full h-[1px] bg-bfc-border"></div>
				<div className="p-2.5 flex justify-between w-full">
					<Text variant="body" weight="normal" color="bfc-text1">
						Gas Fees
					</Text>
					<Text variant="body" weight="normal" color="bfc-text1">
						{gasBudget} {symbol}
					</Text>
				</div>
			</Card>
		</Form>
	);
}

export default memo(StakeForm);
