// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Button } from '_app/shared/ButtonUI';
import { InputWithActionButton } from '_app/shared/InputWithAction';
import { Text } from '_app/shared/text';
import Alert from '_components/alert';
import { AssetData } from '_pages/swap/AssetData';
import { SUI_CONVERSION_RATE, USDC_CONVERSION_RATE, type FormValues } from '_pages/swap/constants';
import { MaxSlippage, MaxSlippageModal } from '_pages/swap/MaxSlippage';
import { getUSDCurrency, useSwapData } from '_pages/swap/utils';
import { type BalanceChange } from '@benfen/bfc.js/client';
import { BFC_TYPE_ARG } from '@benfen/bfc.js/utils';
import BigNumber from 'bignumber.js';
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { useFormContext } from 'react-hook-form';

export function ToAssetSection({
	activeCoinType,
	balanceChanges,
	slippageErrorString,
	baseCoinType,
	quoteCoinType,
	loading,
	refetch,
	error,
}: {
	activeCoinType: string | null;
	balanceChanges: BalanceChange[];
	slippageErrorString: string;
	baseCoinType: string;
	quoteCoinType: string;
	loading: boolean;
	refetch: () => void;
	error: Error | null;
}) {
	const [isSlippageModalOpen, setSlippageModalOpen] = useState(false);
	const isAsk = activeCoinType === BFC_TYPE_ARG;

	const { formattedBaseBalance, formattedQuoteBalance, baseCoinMetadata, quoteCoinMetadata } =
		useSwapData({
			baseCoinType,
			quoteCoinType,
		});

	const toAssetBalance = isAsk ? formattedQuoteBalance : formattedBaseBalance;
	const toAssetMetaData = isAsk ? quoteCoinMetadata : baseCoinMetadata;

	const {
		watch,
		setValue,
		formState: { isValid },
	} = useFormContext<FormValues>();
	const toAssetType = watch('toAssetType');

	const rawToAssetAmount = balanceChanges.find(
		(balanceChange) => balanceChange.coinType === toAssetType,
	)?.amount;

	const toAssetAmountAsNum = new BigNumber(rawToAssetAmount || '0')
		.shiftedBy(isAsk ? -SUI_CONVERSION_RATE : -USDC_CONVERSION_RATE)
		.toNumber();

	useEffect(() => {
		const newToAsset = BFC_TYPE_ARG;
		setValue('toAssetType', newToAsset);
	}, [isAsk, setValue]);

	const toAssetSymbol = toAssetMetaData.data?.symbol ?? '';
	const amount = watch('amount');

	if (!toAssetMetaData.data) {
		return null;
	}

	return (
		<div
			className={clsx(
				'flex flex-col border border-hero-darkest/20 rounded-xl p-5 gap-4 border-solid',
				{ 'bg-sui-primaryBlue2023/10': isValid },
			)}
		>
			<AssetData
				disabled
				tokenBalance={toAssetBalance}
				coinType={toAssetType}
				symbol={toAssetSymbol}
			/>

			<InputWithActionButton
				name="output-amount"
				disabled
				noBorder={!isValid}
				placeholder="--"
				value={toAssetAmountAsNum || '--'}
				loading={loading}
				loadingText="Calculating..."
				suffix={
					!!toAssetAmountAsNum &&
					!loading && (
						<Text variant="body" weight="semibold" color="steel">
							{toAssetSymbol}
						</Text>
					)
				}
				info={
					isValid && (
						<Text variant="subtitleSmall" color="steel-dark">
							{getUSDCurrency(isAsk ? toAssetAmountAsNum : Number(amount))}
						</Text>
					)
				}
			/>

			{isValid && toAssetAmountAsNum && amount ? (
				<div className="ml-3">
					<MaxSlippage onOpen={() => setSlippageModalOpen(true)} />

					{slippageErrorString && (
						<div className="mt-2">
							<Alert>{slippageErrorString}</Alert>
						</div>
					)}

					<MaxSlippageModal
						isOpen={isSlippageModalOpen}
						onClose={() => setSlippageModalOpen(false)}
					/>
				</div>
			) : null}

			{error && (
				<div className="flex flex-col gap-4">
					<Alert>
						<Text variant="pBody" weight="semibold">
							Calculation failed
						</Text>
						<Text variant="pBodySmall">{error.message || 'An error has occurred, try again.'}</Text>
					</Alert>
					<Button text="Recalculate" onClick={refetch} />
				</div>
			)}
		</div>
	);
}
