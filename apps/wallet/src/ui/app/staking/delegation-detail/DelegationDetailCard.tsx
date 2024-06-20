// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BottomMenuLayout, { Content } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import { Card } from '_app/shared/card';
import { Text } from '_app/shared/text';
import { IconTooltip } from '_app/shared/tooltip';
import Alert from '_components/alert';
import LoadingIndicator from '_components/loading/LoadingIndicator';
import { useAppSelector, useCoinsReFetchingConfig } from '_hooks';
import { ampli } from '_src/shared/analytics/ampli';
import { API_ENV } from '_src/shared/api-env';
import { MIN_NUMBER_SUI_TO_STAKE } from '_src/shared/constants';
import FaucetRequestButton from '_src/ui/app/shared/faucet/FaucetRequestButton';
import type { StakeObject } from '@benfen/bfc.js/client';
import { MIST_PER_SUI, SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import {
	useCoinMetadata,
	useGetCoinBalance,
	useGetSystemState,
	useGetValidatorsApy,
} from '@mysten/core';
import { ArrowLeft16 } from '@mysten/icons';
import BigNumber from 'bignumber.js';
import { useMemo } from 'react';

import { useActiveAddress } from '../../hooks/useActiveAddress';
import { getDelegationDataByStakeId } from '../getDelegationByStakeId';
import { StakeAmount } from '../home/StakeAmount';
import { useGetDelegatedStake } from '../useGetDelegatedStake';

type DelegationDetailCardProps = {
	validatorAddress: string;
	stakedId: string;
};

export function DelegationDetailCard({ validatorAddress, stakedId }: DelegationDetailCardProps) {
	const {
		data: system,
		isLoading: loadingValidators,
		isError: errorValidators,
	} = useGetSystemState();

	const accountAddress = useActiveAddress();

	const { data: allDelegation, isLoading, isError } = useGetDelegatedStake(accountAddress || '');

	const apiEnv = useAppSelector(({ app }) => app.apiEnv);
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();
	const { data: suiCoinBalance } = useGetCoinBalance(
		SUI_TYPE_ARG,
		accountAddress,
		refetchInterval,
		staleTime,
	);
	const { data: metadata } = useCoinMetadata(SUI_TYPE_ARG);
	// set minimum stake amount to 1 SUI
	const showRequestMoreSuiToken = useMemo(() => {
		if (!suiCoinBalance?.totalBalance || !metadata?.decimals || apiEnv === API_ENV.mainnet)
			return false;
		const currentBalance = new BigNumber(suiCoinBalance.totalBalance);
		const minStakeAmount = new BigNumber(MIN_NUMBER_SUI_TO_STAKE).shiftedBy(metadata.decimals);
		return currentBalance.lt(minStakeAmount.toString());
	}, [apiEnv, metadata?.decimals, suiCoinBalance?.totalBalance]);

	const { data: rollingAverageApys } = useGetValidatorsApy();

	const validatorData = useMemo(() => {
		if (!system) return null;
		return system.activeValidators.find((av) => av.suiAddress === validatorAddress);
	}, [validatorAddress, system]);

	const delegationData = useMemo(() => {
		return allDelegation ? getDelegationDataByStakeId(allDelegation, stakedId) : null;
	}, [allDelegation, stakedId]);

	const totalStake = BigInt(delegationData?.principal || 0n);

	const suiEarned = BigInt(
		(delegationData as Extract<StakeObject, { estimatedReward: string }>)?.estimatedReward || 0n,
	);
	const { apy, isApyApproxZero } = rollingAverageApys?.[validatorAddress] ?? {
		apy: 0,
	};

	const delegationId = delegationData?.status === 'Active' && delegationData?.stakedSuiId;

	const stakeByValidatorAddress = `/stake/new?${new URLSearchParams({
		address: validatorAddress,
		staked: stakedId,
	}).toString()}`;

	// check if the validator is in the active validator list, if not, is inactive validator
	const hasInactiveValidatorDelegation = !system?.activeValidators?.find(
		({ stakingPoolId }) => stakingPoolId === validatorData?.stakingPoolId,
	);

	const commission = validatorData ? Number(validatorData.commissionRate) / 100 : 0;

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
					<div className="mb-1 font-semibold">Something went wrong</div>
				</Alert>
			</div>
		);
	}

	return (
		<div className="flex flex-col flex-nowrap flex-grow h-full">
			<BottomMenuLayout>
				<Content>
					<div className="justify-center w-full flex flex-col items-center">
						{hasInactiveValidatorDelegation ? (
							<div className="mb-3">
								<Alert>
									Unstake BFC from this inactive validator and stake on an active validator to start
									earning rewards again.
								</Alert>
							</div>
						) : null}
						<div className="w-full flex">
							<Card
								header={
									<div className="grid grid-cols-2 divide-x divide-solid divide-bfc-border divide-y-0 w-full">
										<div className="py-5 px-2.5 flex flex-col gap-1.25 items-center justify-center w-full">
											<Text variant="body" weight="normal" color="bfc-text2">
												Your Stake
											</Text>
											<StakeAmount balance={totalStake} variant="body" isEarnedRewards />
										</div>
										<div className="py-5 px-2.5 flex flex-col gap-1.25 items-center justify-center w-full">
											<Text variant="body" weight="normal" color="bfc-text2">
												Earned
											</Text>
											<StakeAmount balance={suiEarned} variant="body" isEarnedRewards />
										</div>
									</div>
								}
								padding="none"
							>
								<div className="divide-x flex divide-solid divide-bfc-border divide-y-0">
									<div className="py-5 px-2.5 flex flex-col gap-1.25 items-center justify-center w-full">
										<div className="flex text-bfc-text2 gap-1.25 items-center">
											APY
											<IconTooltip tip="Annual Percentage Yield" />
										</div>

										<div className="flex gap-1.25 items-center">
											<Text variant="body" weight="medium" color="bfc-text1">
												{isApyApproxZero ? '~' : ''}
												{apy}
											</Text>
											<Text variant="body" weight="medium" color="bfc-text3">
												%
											</Text>
										</div>
									</div>
									<div className="py-5 px-2.5 flex flex-col gap-1.25 items-center justify-center w-full">
										<div className="flex text-bfc-text2 gap-1.25 items-center">
											Commission
											<IconTooltip tip="Validator commission" />
										</div>
										<div className="flex gap-1.25 items-center">
											<Text variant="body" weight="medium" color="bfc-text1">
												{commission}
											</Text>
											<Text variant="body" weight="medium" color="bfc-text3">
												%
											</Text>
										</div>
									</div>
								</div>
							</Card>
						</div>
						<div className="flex gap-2.5 w-full my-5">
							{!hasInactiveValidatorDelegation ? (
								<Button
									size="tall"
									variant="outline"
									to={stakeByValidatorAddress}
									before={
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="15"
											height="14"
											viewBox="0 0 15 14"
											fill="none"
										>
											<path
												d="M8.66624 1.16675C9.73372 1.16624 10.7691 1.53172 11.5997 2.20224C12.4304 2.87276 13.006 3.80778 13.2306 4.85135C13.4553 5.89493 13.3153 6.98399 12.8342 7.93688C12.353 8.88977 11.5597 9.6489 10.5866 10.0877C10.2755 10.7754 9.80229 11.3773 9.20749 11.842C8.61269 12.3066 7.91406 12.6201 7.17153 12.7554C6.42899 12.8908 5.6647 12.844 4.9442 12.6191C4.22371 12.3943 3.56851 11.998 3.0348 11.4643C2.5011 10.9306 2.10481 10.2754 1.87992 9.55487C1.65504 8.83438 1.60827 8.07008 1.74362 7.32755C1.87898 6.58501 2.19242 5.88638 2.65707 5.29158C3.12172 4.69678 3.72371 4.22356 4.41141 3.9125C4.78132 3.09402 5.3795 2.39962 6.13419 1.9126C6.88888 1.42558 7.76805 1.16661 8.66624 1.16675ZM6.33291 4.66675C5.87328 4.66675 5.41815 4.75728 4.99351 4.93317C4.56887 5.10906 4.18304 5.36687 3.85803 5.69187C3.53303 6.01688 3.27522 6.40272 3.09933 6.82736C2.92344 7.252 2.83291 7.70712 2.83291 8.16675C2.83291 8.62638 2.92344 9.0815 3.09933 9.50614C3.27522 9.93078 3.53303 10.3166 3.85803 10.6416C4.18304 10.9666 4.56887 11.2244 4.99351 11.4003C5.41815 11.5762 5.87328 11.6667 6.33291 11.6667C7.26116 11.6667 8.1514 11.298 8.80778 10.6416C9.46416 9.98524 9.83291 9.09501 9.83291 8.16675C9.83291 7.23849 9.46416 6.34825 8.80778 5.69187C8.1514 5.0355 7.26116 4.66675 6.33291 4.66675ZM8.66624 2.33342C8.17197 2.33284 7.68322 2.43721 7.2323 2.63964C6.78139 2.84207 6.3786 3.13794 6.05057 3.50767C6.70989 3.46764 7.3702 3.56804 7.98783 3.80222C8.60546 4.0364 9.16633 4.39902 9.63337 4.86612C10.1004 5.33322 10.463 5.89414 10.6971 6.51179C10.9312 7.12945 11.0315 7.78977 10.9914 8.44908C11.5218 7.97753 11.8963 7.35588 12.0652 6.6666C12.2342 5.97731 12.1896 5.25295 11.9374 4.58959C11.6852 3.92622 11.2372 3.3552 10.653 2.95226C10.0688 2.54932 9.37593 2.3335 8.66624 2.33342Z"
												fill="#171719"
											/>
										</svg>
									}
									text="Stake BFC"
									onClick={() => {
										ampli.clickedStakeSui({
											isCurrentlyStaking: true,
											sourceFlow: 'Delegation detail card',
										});
									}}
									disabled={showRequestMoreSuiToken}
								/>
							) : null}

							{Boolean(totalStake) && delegationId && (
								<Button
									data-testid="unstake-button"
									size="tall"
									variant="outline"
									to={stakeByValidatorAddress + '&unstake=true'}
									onClick={() => {
										ampli.clickedUnstakeSui({
											stakedAmount: Number(totalStake / MIST_PER_SUI),
											validatorAddress,
										});
									}}
									text="Unstake BFC"
									before={
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="15"
											height="14"
											viewBox="0 0 15 14"
											fill="none"
										>
											<path
												d="M8.66624 1.16675C9.73372 1.16624 10.7691 1.53172 11.5997 2.20224C12.4304 2.87276 13.006 3.80778 13.2306 4.85135C13.4553 5.89493 13.3153 6.98399 12.8342 7.93688C12.353 8.88977 11.5597 9.6489 10.5866 10.0877C10.2755 10.7754 9.80229 11.3773 9.20749 11.842C8.61269 12.3066 7.91406 12.6201 7.17153 12.7554C6.42899 12.8908 5.6647 12.844 4.9442 12.6191C4.22371 12.3943 3.56851 11.998 3.0348 11.4643C2.5011 10.9306 2.10481 10.2754 1.87992 9.55487C1.65504 8.83438 1.60827 8.07008 1.74362 7.32755C1.87898 6.58501 2.19242 5.88638 2.65707 5.29158C3.12172 4.69678 3.72371 4.22356 4.41141 3.9125C4.78132 3.09402 5.3795 2.39962 6.13419 1.9126C6.88888 1.42558 7.76805 1.16661 8.66624 1.16675ZM6.33291 4.66675C5.87328 4.66675 5.41815 4.75728 4.99351 4.93317C4.56887 5.10906 4.18304 5.36687 3.85803 5.69187C3.53303 6.01688 3.27522 6.40272 3.09933 6.82736C2.92344 7.252 2.83291 7.70712 2.83291 8.16675C2.83291 8.62638 2.92344 9.0815 3.09933 9.50614C3.27522 9.93078 3.53303 10.3166 3.85803 10.6416C4.18304 10.9666 4.56887 11.2244 4.99351 11.4003C5.41815 11.5762 5.87328 11.6667 6.33291 11.6667C7.26116 11.6667 8.1514 11.298 8.80778 10.6416C9.46416 9.98524 9.83291 9.09501 9.83291 8.16675C9.83291 7.23849 9.46416 6.34825 8.80778 5.69187C8.1514 5.0355 7.26116 4.66675 6.33291 4.66675ZM8.66624 2.33342C8.17197 2.33284 7.68322 2.43721 7.2323 2.63964C6.78139 2.84207 6.3786 3.13794 6.05057 3.50767C6.70989 3.46764 7.3702 3.56804 7.98783 3.80222C8.60546 4.0364 9.16633 4.39902 9.63337 4.86612C10.1004 5.33322 10.463 5.89414 10.6971 6.51179C10.9312 7.12945 11.0315 7.78977 10.9914 8.44908C11.5218 7.97753 11.8963 7.35588 12.0652 6.6666C12.2342 5.97731 12.1896 5.25295 11.9374 4.58959C11.6852 3.92622 11.2372 3.3552 10.653 2.95226C10.0688 2.54932 9.37593 2.3335 8.66624 2.33342Z"
												fill="#171719"
											/>
										</svg>
									}
								/>
							)}
						</div>
					</div>
				</Content>

				{/* show faucet request button on devnet or testnet whenever there is only one coin  */}
				{showRequestMoreSuiToken ? (
					<div className="flex flex-col gap-4 items-center">
						<div className="w-8/12 text-center">
							<Text variant="pSubtitle" weight="medium" color="steel-darker">
								You need a minimum of {MIN_NUMBER_SUI_TO_STAKE} BFC to continue staking.
							</Text>
						</div>
						<FaucetRequestButton size="tall" />
					</div>
				) : (
					<Button
						size="tall"
						variant="secondary"
						to="/stake"
						before={<ArrowLeft16 />}
						text="Back"
					/>
				)}
			</BottomMenuLayout>
		</div>
	);
}
