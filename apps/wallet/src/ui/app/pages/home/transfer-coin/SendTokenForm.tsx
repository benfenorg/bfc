// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useActiveAddress } from '_app/hooks/useActiveAddress';
import BottomMenuLayout, { Content, Menu } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import { Text } from '_app/shared/text';
import { AddressInput } from '_components/address-input';
import Alert from '_components/alert';
import Loading from '_components/loading';
import { parseAmount } from '_helpers';
import { useGetAllCoins } from '_hooks';
import { GAS_SYMBOL } from '_src/ui/app/redux/slices/sui-objects/Coin';
import { InputWithAction } from '_src/ui/app/shared/InputWithAction';
import { type CoinStruct } from '@benfen/bfc.js/client';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import {
	CoinFormat,
	isSuiNSName,
	useCoinMetadata,
	useFormatCoin,
	useSuiNSEnabled,
} from '@mysten/core';
import { ArrowRight16 } from '@mysten/icons';
import { useQuery } from '@tanstack/react-query';
import cl from 'classnames';
import { Field, Form, Formik, useFormikContext } from 'formik';
import { useEffect, useMemo } from 'react';

import { createTokenTransferTransaction } from './utils/transaction';
import { createValidationSchemaStepOne } from './validation';

const initialValues = {
	to: '',
	amount: '',
	isPayAllSui: false,
	gasBudgetEst: '',
};

export type FormValues = typeof initialValues;

export type SubmitProps = {
	to: string;
	amount: string;
	isPayAllSui: boolean;
	coinIds: string[];
	coins: CoinStruct[];
	gasBudgetEst: string;
};

export type SendTokenFormProps = {
	coinType: string;
	onSubmit: (values: SubmitProps) => void;
	initialAmount: string;
	initialTo: string;
};

function totalBalance(coins: CoinStruct[]): bigint {
	return coins.reduce((partialSum, c) => partialSum + getBalanceFromCoinStruct(c), BigInt(0));
}
function getBalanceFromCoinStruct(coin: CoinStruct): bigint {
	return BigInt(coin.balance);
}

function GasBudgetEstimation({
	coinDecimals,
	coins,
}: {
	coinDecimals: number;
	coins: CoinStruct[];
}) {
	const activeAddress = useActiveAddress();
	const { values, setFieldValue } = useFormikContext<FormValues>();
	const suiNSEnabled = useSuiNSEnabled();

	const rpc = useSuiClient();
	const { data: gasBudget } = useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: [
			'transaction-gas-budget-estimate',
			{
				to: values.to,
				amount: values.amount,
				coins,
				activeAddress,
				coinDecimals,
			},
		],
		queryFn: async () => {
			if (!values.amount || !values.to || !coins || !activeAddress) {
				return null;
			}

			let to = values.to;
			if (suiNSEnabled && isSuiNSName(values.to)) {
				const address = await rpc.resolveNameServiceAddress({
					name: values.to,
				});
				if (!address) {
					throw new Error('SuiNS name not found.');
				}
				to = address;
			}

			const tx = createTokenTransferTransaction({
				to,
				amount: values.amount,
				coinType: SUI_TYPE_ARG,
				coinDecimals,
				isPayAllSui: values.isPayAllSui,
				coins,
			});

			tx.setSender(activeAddress);
			await tx.build({ client: rpc });
			return tx.blockData.gasConfig.budget;
		},
	});

	const [formattedGas] = useFormatCoin(gasBudget, SUI_TYPE_ARG);

	// gasBudgetEstimation should change when the amount above changes
	useEffect(() => {
		setFieldValue('gasBudgetEst', formattedGas, true);
	}, [formattedGas, setFieldValue, values.amount]);

	return (
		<div className="mt-1.25 flex w-full gap-2 justify-between">
			<div className="grow">
				<Text variant="body" color="bfc-text1" weight="normal">
					Estimated Gas Fees
				</Text>
			</div>
			<Text variant="body" color="bfc-text1" weight="normal">
				{formattedGas ? formattedGas + ' ' + GAS_SYMBOL : '--'}
			</Text>
		</div>
	);
}

// Set the initial gasEstimation from initial amount
// base on the input amount field update the gasEstimation value
// Separating the gasEstimation from the formik context to access the input amount value and update the gasEstimation value
export function SendTokenForm({
	coinType,
	onSubmit,
	initialAmount = '',
	initialTo = '',
}: SendTokenFormProps) {
	const rpc = useSuiClient();
	const activeAddress = useActiveAddress();
	// Get all coins of the type
	const { data: coinsData, isLoading: coinsIsLoading } = useGetAllCoins(coinType, activeAddress!);

	const { data: suiCoinsData, isLoading: suiCoinsIsLoading } = useGetAllCoins(
		SUI_TYPE_ARG,
		activeAddress!,
	);

	const suiCoins = suiCoinsData;
	const coins = coinsData;
	const coinBalance = totalBalance(coins || []);
	const suiBalance = totalBalance(suiCoins || []);

	const coinMetadata = useCoinMetadata(coinType);
	const coinDecimals = coinMetadata.data?.decimals ?? 0;

	const [tokenBalance, symbol, queryResult] = useFormatCoin(coinBalance, coinType, CoinFormat.FULL);
	const suiNSEnabled = useSuiNSEnabled();

	const validationSchemaStepOne = useMemo(
		() => createValidationSchemaStepOne(rpc, suiNSEnabled, coinBalance, symbol, coinDecimals),
		[rpc, coinBalance, symbol, coinDecimals, suiNSEnabled],
	);

	// remove the comma from the token balance
	const formattedTokenBalance = tokenBalance.replace(/,/g, '');
	const initAmountBig = parseAmount(initialAmount, coinDecimals);

	return (
		<Loading
			loading={
				queryResult.isLoading || coinMetadata.isLoading || suiCoinsIsLoading || coinsIsLoading
			}
		>
			<Formik
				initialValues={{
					amount: initialAmount,
					to: initialTo,
					isPayAllSui:
						!!initAmountBig && initAmountBig === coinBalance && coinType === SUI_TYPE_ARG,
					gasBudgetEst: '',
				}}
				validationSchema={validationSchemaStepOne}
				enableReinitialize
				validateOnMount
				validateOnChange
				onSubmit={async ({ to, amount, isPayAllSui, gasBudgetEst }: FormValues) => {
					if (!coins || !suiCoins) return;
					const coinsIDs = [...coins]
						.sort((a, b) => Number(b.balance) - Number(a.balance))
						.map(({ coinObjectId }) => coinObjectId);

					if (suiNSEnabled && isSuiNSName(to)) {
						const address = await rpc.resolveNameServiceAddress({
							name: to,
						});
						if (!address) {
							throw new Error('SuiNS name not found.');
						}
						to = address;
					}

					const data = {
						to,
						amount,
						isPayAllSui,
						coins,
						coinIds: coinsIDs,
						gasBudgetEst,
					};
					onSubmit(data);
				}}
			>
				{({ isValid, isSubmitting, setFieldValue, values, submitForm, validateField }) => {
					const newPaySuiAll =
						parseAmount(values.amount, coinDecimals) === coinBalance && coinType === SUI_TYPE_ARG;
					if (values.isPayAllSui !== newPaySuiAll) {
						setFieldValue('isPayAllSui', newPaySuiAll);
					}

					const hasEnoughBalance =
						values.isPayAllSui ||
						suiBalance >
							parseAmount(values.gasBudgetEst, coinDecimals) +
								parseAmount(coinType === SUI_TYPE_ARG ? values.amount : '0', coinDecimals);
					const actionDisabled =
						parseAmount(values?.amount, coinDecimals) === coinBalance ||
						queryResult.isLoading ||
						!coinBalance;

					return (
						<BottomMenuLayout>
							<Content>
								<Form autoComplete="off" noValidate>
									<div className="w-full flex flex-col flex-grow gap-1.25">
										<Text variant="body" color="bfc-text2" weight="normal">
											Select Coin Amount to Send
										</Text>

										<InputWithAction
											data-testid="coin-amount-input"
											type="numberInput"
											name="amount"
											placeholder="0.00"
											prefix={values.isPayAllSui ? '~ ' : ''}
											actionText="Max"
											actionClass={cl('border border-solid rounded-[30px]', {
												'border-bfc-text1': !actionDisabled,
												'border-bfc-text3': actionDisabled,
											})}
											suffix={` ${symbol}`}
											actionType="button"
											allowNegative={false}
											decimals
											rounded="lg"
											dark
											onActionClicked={async () => {
												// using await to make sure the value is set before the validation
												await setFieldValue('amount', formattedTokenBalance);
												validateField('amount');
											}}
											actionDisabled={actionDisabled}
										/>
									</div>
									{!hasEnoughBalance && isValid ? (
										<div>
											<Alert>Insufficient BFC to cover transaction</Alert>
										</div>
									) : null}

									{coins ? <GasBudgetEstimation coinDecimals={coinDecimals} coins={coins} /> : null}

									<div className="w-full flex gap-2.5 flex-col mt-7.5">
										<Text variant="body" color="bfc-text2" weight="normal">
											Enter Recipient Address
										</Text>
										<div className="w-full flex relative items-center flex-col">
											<Field component={AddressInput} name="to" placeholder="Enter Address" />
										</div>
									</div>
								</Form>
							</Content>
							<Menu stuckClass="sendCoin-cta" className="w-full px-0 pb-0 mx-0 gap-2.5">
								<Button
									type="submit"
									onClick={submitForm}
									variant="primary"
									loading={isSubmitting}
									disabled={
										!isValid || isSubmitting || !hasEnoughBalance || values.gasBudgetEst === ''
									}
									size="tall"
									text="Review"
									after={<ArrowRight16 />}
								/>
							</Menu>
						</BottomMenuLayout>
					);
				}}
			</Formik>
		</Loading>
	);
}
