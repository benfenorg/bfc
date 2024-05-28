// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { formatAddress, SUI_TYPE_ARG } from '@benfen/bfc.js';
import { useFormatCoin, useGetCoinBalance, useResolveSuiNSName } from '@mysten/core';
import { CheckFill16 } from '@mysten/icons';
import cl from 'classnames';
=======
import { Text } from '_src/ui/app/shared/text';
import { useFormatCoin, useResolveSuiNSName } from '@mysten/core';
import { useSuiClientQuery } from '@mysten/dapp-kit';
import { CheckFill16 } from '@mysten/icons';
import { formatAddress, SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import cl from 'clsx';
>>>>>>> mainnet-v1.24.1

import { useCoinsReFetchingConfig } from '../../hooks';

type LedgerAccountRowProps = {
	isSelected: boolean;
	address: string;
};

export function LedgerAccountRow({ isSelected, address }: LedgerAccountRowProps) {
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();
<<<<<<< HEAD
	const { data: coinBalance } = useGetCoinBalance(
		SUI_TYPE_ARG,
		address,
		refetchInterval,
		staleTime,
=======

	const { data: coinBalance } = useSuiClientQuery(
		'getBalance',
		{
			coinType: SUI_TYPE_ARG,
			owner: address,
		},
		{
			refetchInterval,
			staleTime,
		},
>>>>>>> mainnet-v1.24.1
	);
	const { data: domainName } = useResolveSuiNSName(address);
	const [totalAmount, totalAmountSymbol] = useFormatCoin(
		coinBalance?.totalBalance ?? 0,
		SUI_TYPE_ARG,
	);

	return (
		<div className="flex items-center gap-3">
			<CheckFill16
				className={cl('w-4 h-4', {
					'text-gray-50': !isSelected,
					'text-success': isSelected,
				})}
			/>
			<Text
				mono
				variant="bodySmall"
				weight="semibold"
				color={isSelected ? 'steel-darker' : 'steel-dark'}
			>
				{domainName ?? formatAddress(address)}
			</Text>
			<div className="ml-auto">
				<Text variant="bodySmall" color="steel" weight="semibold" mono>
					{totalAmount} {totalAmountSymbol}
				</Text>
			</div>
		</div>
	);
}
