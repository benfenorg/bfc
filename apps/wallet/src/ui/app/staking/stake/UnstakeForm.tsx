// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Card } from '_app/shared/card';
import { Text } from '_app/shared/text';
import { CountDownTimer } from '_src/ui/app/shared/countdown-timer';
import { SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { useFormatCoin, useGetTimeBeforeEpochNumber } from '@mysten/core';
import { Form } from 'formik';
import { useMemo } from 'react';

import { useActiveAddress, useTransactionGasBudget } from '../../hooks';
import { GAS_SYMBOL } from '../../redux/slices/sui-objects/Coin';
import { Heading } from '../../shared/heading';
import { createUnstakeTransaction } from './utils/transaction';

export type StakeFromProps = {
	stakedSuiId: string;
	coinBalance: bigint;
	coinType: string;
	stakingReward?: string;
	epoch: number;
};

export function UnStakeForm({
	stakedSuiId,
	coinBalance,
	coinType,
	stakingReward,
	epoch,
}: StakeFromProps) {
	const [rewards, rewardSymbol] = useFormatCoin(stakingReward, SUI_TYPE_ARG);
	const [totalSui] = useFormatCoin(BigInt(stakingReward || 0) + coinBalance, SUI_TYPE_ARG);
	const [tokenBalance] = useFormatCoin(coinBalance, coinType);

	const transaction = useMemo(() => createUnstakeTransaction(stakedSuiId), [stakedSuiId]);
	const activeAddress = useActiveAddress();
	const { data: gasBudget } = useTransactionGasBudget(activeAddress, transaction);

	const { data: currentEpochEndTime } = useGetTimeBeforeEpochNumber(epoch + 1 || 0);

	return (
		<Form className="flex flex-1 flex-col flex-nowrap" autoComplete="off" noValidate>
			<Card>
				<div className="h-10 px-2.5 w-full flex items-center bg-bfc-card justify-between">
					<Text variant="body" weight="medium" color="bfc-text1">
						Current Epoch Ends
					</Text>
					<div className="flex gap-0.5 ml-auto">
						{currentEpochEndTime > 0 ? (
							<CountDownTimer
								timestamp={currentEpochEndTime}
								variant="body"
								color="bfc-text1"
								weight="normal"
								endLabel="--"
							/>
						) : (
							<Text variant="body" weight="normal" color="bfc-text1">
								Epoch #{epoch}
							</Text>
						)}
					</div>
				</div>
				<div className="p-2.5 flex flex-col w-full gap-2.5">
					<div className="flex gap-0.5 justify-between w-full">
						<Text variant="body" weight="normal" color="bfc-text2">
							Your Stake
						</Text>
						<Text variant="body" weight="normal" color="bfc-text2">
							{tokenBalance} {GAS_SYMBOL}
						</Text>
					</div>
					<div className="flex gap-0.5 justify-between w-full">
						<Text variant="body" weight="normal" color="bfc-text2">
							Staking Rewards Earned
						</Text>
						<Text variant="body" weight="normal" color="bfc-text2">
							{rewards} {rewardSymbol}
						</Text>
					</div>
				</div>
				<div className="mx-2.5 h-px bg-bfc-card"></div>
				<div className="h-10 px-2.5 flex items-center gap-0.5 justify-between w-full">
					<Text variant="body" weight="normal" color="bfc-text2">
						Total unstaked BFC
					</Text>
					<div className="flex gap-0.5 ml-auto">
						<Heading variant="heading4" weight="semibold" color="bfc-text1" leading="none">
							{totalSui}
						</Heading>
						<Text variant="body" weight="normal" color="bfc-text1">
							{GAS_SYMBOL}
						</Text>
					</div>
				</div>
			</Card>
			<div className="mt-5 p-2.5 flex justify-between w-full bg-bfc-card border border-solid border-bfc-border rounded-lg">
				<Text variant="body" weight="normal" color="bfc-text1">
					Gas Fees
				</Text>
				<Text variant="body" weight="normal" color="bfc-text1">
					{gasBudget || '-'} {GAS_SYMBOL}
				</Text>
			</div>
		</Form>
	);
}
