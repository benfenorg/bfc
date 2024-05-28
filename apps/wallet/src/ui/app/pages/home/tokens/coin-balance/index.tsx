// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
<<<<<<< HEAD

import { useFormatCoin } from '@mysten/core';
=======
import { useIsWalletDefiEnabled } from '_app/hooks/useIsWalletDefiEnabled';
import { useAppSelector } from '_hooks';
import { API_ENV } from '_shared/api-env';
import { Heading } from '_src/ui/app/shared/heading';
import { Text } from '_src/ui/app/shared/text';
import { useBalanceInUSD, useFormatCoin } from '@mysten/core';
import { SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import { useMemo } from 'react';
>>>>>>> mainnet-v1.24.1

export type CoinProps = {
	type: string;
	balance: bigint;
};

<<<<<<< HEAD
export default function CoinBalance({ type, balance }: CoinProps) {
	const [formatted, symbol] = useFormatCoin(balance, type);

	return (
		<div className="flex items-end gap-1.25" role="button">
			<span className="text-[32px] text-bfc font-bold">{formatted}</span>
			<span className="text-[20px]/[32px] text-bfc-text3 font-bold">{symbol}</span>
=======
function WalletBalanceUsd({ amount: walletBalance }: { amount: bigint }) {
	const isDefiWalletEnabled = useIsWalletDefiEnabled();
	const formattedWalletBalance = useBalanceInUSD(SUI_TYPE_ARG, walletBalance);

	const walletBalanceInUsd = useMemo(() => {
		if (!formattedWalletBalance) return null;

		return `~${formattedWalletBalance.toLocaleString('en', {
			style: 'currency',
			currency: 'USD',
		})} USD`;
	}, [formattedWalletBalance]);

	if (!walletBalanceInUsd) {
		return null;
	}

	return (
		<Text variant="caption" weight="medium" color={isDefiWalletEnabled ? 'hero-darkest' : 'steel'}>
			{walletBalanceInUsd}
		</Text>
	);
}

export function CoinBalance({ amount: walletBalance, type }: CoinProps) {
	const { apiEnv } = useAppSelector((state) => state.app);
	const [formatted, symbol] = useFormatCoin(walletBalance, type);

	return (
		<div className="flex flex-col gap-1 items-center justify-center">
			<div className="flex items-center justify-center gap-2">
				<Heading leading="none" variant="heading1" weight="bold" color="gray-90">
					{formatted}
				</Heading>

				<Heading variant="heading6" weight="medium" color="steel">
					{symbol}
				</Heading>
			</div>
			<div>{apiEnv === API_ENV.mainnet ? <WalletBalanceUsd amount={walletBalance} /> : null}</div>
>>>>>>> mainnet-v1.24.1
		</div>
	);
}
