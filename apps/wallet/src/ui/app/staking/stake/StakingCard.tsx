// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useActiveAddress } from '_app/hooks/useActiveAddress';
import BottomMenuLayout, { Content, Menu } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import { Collapse } from '_app/shared/collapse';
import { Text } from '_app/shared/text';
import Loading from '_components/loading';
import { parseAmount } from '_helpers';
import { useCoinsReFetchingConfig } from '_hooks';
import { Coin } from '_redux/slices/sui-objects/Coin';
import { ampli } from '_src/shared/analytics/ampli';
import { MIN_NUMBER_SUI_TO_STAKE } from '_src/shared/constants';
import type { StakeObject } from '@benfen/bfc.js/client';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { MIST_PER_SUI, SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { Popover } from '@headlessui/react';
import { useCoinMetadata, useGetAllBalances, useGetSystemState } from '@mysten/core';
import { ArrowLeft16 } from '@mysten/icons';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Formik } from 'formik';
import type { FormikHelpers } from 'formik';
import { useCallback, useMemo, useState } from 'react';
import { toast } from 'react-hot-toast';
import { Navigate, useNavigate, useSearchParams } from 'react-router-dom';

import Alert from '../../components/alert';
import { getSignerOperationErrorMessage } from '../../helpers/errorMessages';
import { useQredoTransaction } from '../../hooks/useQredoTransaction';
import { useSigner } from '../../hooks/useSigner';
import { QredoActionIgnoredByUser } from '../../QredoSigner';
import { getDelegationDataByStakeId } from '../getDelegationByStakeId';
import { getStakeSuiBySuiId } from '../getStakeSuiBySuiId';
import { useGetDelegatedStake } from '../useGetDelegatedStake';
import StakeForm from './StakeForm';
import { UnStakeForm } from './UnstakeForm';
import { createStakeTransaction, createUnstakeTransaction } from './utils/transaction';
import { createValidationSchema } from './utils/validation';
import { ValidatorFormDetail } from './ValidatorFormDetail';

const initialValues = {
	amount: '',
};

export type FormValues = typeof initialValues;

function StakingCard() {
	const suiClient = useSuiClient();
	const [coinType, setCoinType] = useState(SUI_TYPE_ARG);

	const accountAddress = useActiveAddress();
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();
	const { data, isLoading: loadingSuiBalances } = useGetAllBalances(
		accountAddress,
		refetchInterval,
		staleTime,
	);

	const coinData = data?.find((coin) => coin.coinType === coinType);
	const coinBalance = BigInt(coinData?.totalBalance || 0);
	const [searchParams] = useSearchParams();
	const validatorAddress = searchParams.get('address');
	const stakeSuiIdParams = searchParams.get('staked');
	const unstake = searchParams.get('unstake') === 'true';
	const { data: allDelegation, isLoading } = useGetDelegatedStake(accountAddress || '');

	const { data: system, isLoading: validatorsIsloading } = useGetSystemState();

	const totalTokenBalance = useMemo(() => {
		if (!allDelegation) return 0n;
		// return only the total amount of tokens staked for a specific stakeId
		return getStakeSuiBySuiId(allDelegation, stakeSuiIdParams);
	}, [allDelegation, stakeSuiIdParams]);

	const stakeData = useMemo(() => {
		if (!allDelegation || !stakeSuiIdParams) return null;
		// return delegation data for a specific stakeId
		return getDelegationDataByStakeId(allDelegation, stakeSuiIdParams);
	}, [allDelegation, stakeSuiIdParams]);

	const coinSymbol = useMemo(() => (coinType && Coin.getCoinSymbol(coinType)) || '', [coinType]);

	const suiEarned =
		(stakeData as Extract<StakeObject, { estimatedReward: string }>)?.estimatedReward || '0';

	const { data: metadata } = useCoinMetadata(coinType);
	const coinDecimals = metadata?.decimals ?? 0;
	// set minimum stake amount to 1 SUI
	const minimumStake = parseAmount(MIN_NUMBER_SUI_TO_STAKE.toString(), coinDecimals);

	const validationSchema = useMemo(
		() => createValidationSchema(coinBalance, coinSymbol, coinDecimals, unstake, minimumStake),
		[coinBalance, coinSymbol, coinDecimals, unstake, minimumStake],
	);

	const queryClient = useQueryClient();
	const delegationId = useMemo(() => {
		if (!stakeData || stakeData.status === 'Pending') return null;
		return stakeData.stakedSuiId;
	}, [stakeData]);

	const navigate = useNavigate();
	const signer = useSigner();
	const { clientIdentifier, notificationModal } = useQredoTransaction();

	const stakeToken = useMutation({
		mutationFn: async ({
			tokenTypeArg,
			amount,
			validatorAddress,
		}: {
			tokenTypeArg: string;
			amount: bigint;
			validatorAddress: string;
		}) => {
			if (!validatorAddress || !amount || !tokenTypeArg || !signer) {
				throw new Error('Failed, missing required field');
			}

			const { data: coins } = await suiClient.getCoins({ owner: accountAddress!, coinType });
			const transactionBlock = createStakeTransaction(amount, validatorAddress, coinType, coins);
			return await signer.signAndExecuteTransactionBlock(
				{
					transactionBlock,
					requestType: 'WaitForLocalExecution',
					options: {
						showInput: true,
						showEffects: true,
						showEvents: true,
					},
				},
				clientIdentifier,
			);
		},
		onSuccess: (_, { amount, validatorAddress }) => {
			ampli.stakedSui({
				stakedAmount: Number(amount / MIST_PER_SUI),
				validatorAddress: validatorAddress,
			});
		},
	});

	const unStakeToken = useMutation({
		mutationFn: async ({ stakedSuiId }: { stakedSuiId: string }) => {
			if (!stakedSuiId || !signer) {
				throw new Error('Failed, missing required field.');
			}

			const transactionBlock = createUnstakeTransaction(stakedSuiId);
			return await signer.signAndExecuteTransactionBlock(
				{
					transactionBlock,
					requestType: 'WaitForLocalExecution',
					options: {
						showInput: true,
						showEffects: true,
						showEvents: true,
					},
				},
				clientIdentifier,
			);
		},
		onSuccess: () => {
			ampli.unstakedSui({
				validatorAddress: validatorAddress!,
			});
		},
	});

	const onHandleSubmit = useCallback(
		async ({ amount }: FormValues, { resetForm }: FormikHelpers<FormValues>) => {
			if (coinType === null || validatorAddress === null) {
				return;
			}
			try {
				const bigIntAmount = parseAmount(amount, coinDecimals);
				let response;
				let txDigest;
				if (unstake) {
					// check for delegation data
					if (!stakeData || !stakeSuiIdParams || stakeData.status === 'Pending') {
						return;
					}
					response = await unStakeToken.mutateAsync({
						stakedSuiId: stakeSuiIdParams,
					});

					txDigest = response.digest;
				} else {
					response = await stakeToken.mutateAsync({
						amount: bigIntAmount,
						tokenTypeArg: coinType,
						validatorAddress: validatorAddress,
					});
					txDigest = response.digest;
				}

				// Invalidate the react query for system state and validator
				Promise.all([
					queryClient.invalidateQueries({
						queryKey: ['system', 'state'],
					}),
					queryClient.invalidateQueries({
						queryKey: ['validator'],
					}),
				]);
				resetForm();

				navigate(
					`/receipt?${new URLSearchParams({
						txdigest: txDigest,
						from: 'tokens',
					}).toString()}`,
					response?.transaction
						? {
								state: {
									response,
								},
						  }
						: undefined,
				);
			} catch (error) {
				if (error instanceof QredoActionIgnoredByUser) {
					navigate('/');
				} else {
					toast.error(
						<div className="max-w-xs overflow-hidden flex flex-col">
							<strong>{unstake ? 'Unstake' : 'Stake'} failed</strong>
							<small className="text-ellipsis overflow-hidden">
								{getSignerOperationErrorMessage(error)}
							</small>
						</div>,
					);
				}
			}
		},
		[
			coinType,
			validatorAddress,
			coinDecimals,
			unstake,
			queryClient,
			navigate,
			stakeData,
			stakeSuiIdParams,
			unStakeToken,
			stakeToken,
		],
	);

	if (!coinType || !validatorAddress || (!validatorsIsloading && !system)) {
		return <Navigate to="/" replace={true} />;
	}
	return (
		<div className="flex flex-col flex-nowrap flex-grow w-full">
			<Loading loading={isLoading || validatorsIsloading || loadingSuiBalances}>
				<Formik
					initialValues={initialValues}
					validationSchema={validationSchema}
					onSubmit={onHandleSubmit}
					validateOnMount
				>
					{({ isSubmitting, isValid, submitForm, errors, touched }) => (
						<BottomMenuLayout>
							<Content>
								<Popover className="relative z-10 max-w-full px-5">
									{({ close }) => (
										<>
											<Popover.Button className="cursor-pointer px-2 py-1 rounded border border-bfc-border outline-0 bg-white">
												{coinType}
											</Popover.Button>
											<Popover.Panel className="absolute mt-2 flex flex-col items-center shadow border-bfc-border bg-white">
												{data?.map((coin) => (
													<button
														key={coin.coinType}
														className="w-full px-5 py-2 text-left hover:bg-bfc-hover"
														onClick={() => {
															setCoinType(coin.coinType);
															close();
														}}
													>
														{coin.coinType}
													</button>
												))}
											</Popover.Panel>
										</>
									)}
								</Popover>

								<div className="my-5">
									<ValidatorFormDetail validatorAddress={validatorAddress} unstake={unstake} />
								</div>

								{unstake ? (
									<UnStakeForm
										stakedSuiId={stakeSuiIdParams!}
										coinBalance={totalTokenBalance}
										coinType={coinType}
										stakingReward={suiEarned}
										epoch={Number(system?.epoch || 0)}
									/>
								) : (
									<StakeForm
										validatorAddress={validatorAddress}
										coinBalance={coinBalance}
										coinType={coinType}
										epoch={system?.epoch}
									/>
								)}

								{(unstake || touched.amount) && errors.amount ? (
									<div className="mt-2.5 flex flex-col flex-nowrap">
										<Alert>{errors.amount}</Alert>
									</div>
								) : null}

								{!unstake && (
									<div className="flex-1 mt-5">
										<Collapse title="Staking Rewards" initialIsOpen>
											<Text variant="body" color="bfc-text2" weight="normal">
												Staked BFC starts counting as validator’s stake at the end of the Epoch in
												which it was staked. Rewards are earned separately for each Epoch and become
												available at the end of each Epoch.
											</Text>
										</Collapse>
									</div>
								)}
							</Content>

							<Menu stuckClass="staked-cta" className="w-full px-0 pb-0 mx-0">
								<Button
									size="tall"
									variant="secondary"
									to="/stake"
									disabled={isSubmitting}
									before={<ArrowLeft16 />}
									text="Back"
								/>
								<Button
									size="tall"
									variant="primary"
									onClick={submitForm}
									disabled={!isValid || isSubmitting || (unstake && !delegationId)}
									loading={isSubmitting}
									text={unstake ? 'Unstake Now' : 'Stake Now'}
								/>
							</Menu>
						</BottomMenuLayout>
					)}
				</Formik>
			</Loading>
			{notificationModal}
		</div>
	);
}

export default StakingCard;
