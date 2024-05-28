// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useFormatCoin } from '@mysten/core';
import cl from 'classnames';
import { type ReactNode } from 'react';

=======
>>>>>>> mainnet-v1.24.1
import { Text } from '_app/shared/text';
import { CoinIcon } from '_components/coin-icon';
import { useFormatCoin } from '@mysten/core';
import { type ReactNode } from 'react';

type CoinItemProps = {
	coinType: string;
	balance: bigint;
	isActive?: boolean;
	usd?: number;
	centerAction?: ReactNode;
	subtitle?: string;
};

export function CoinItem({
	coinType,
	balance,
	isActive,
	usd,
	centerAction,
	subtitle,
}: CoinItemProps) {
	const [formatted, symbol, { data: coinMeta }] = useFormatCoin(balance, coinType);

	return (
		<div
			className={cl(
				'flex gap-2.5 w-full p-2.5 justify-center items-center rounded-lg hover:bg-bfc-card',
				{
					'bg-bfc-card': isActive,
				},
			)}
		>
			<CoinIcon coinType={coinType} size={isActive ? 'sm' : 'md'} />
			<div className="flex flex-1 gap-1.5 justify-between items-center">
<<<<<<< HEAD
				<div className="flex flex-col">
					<Text variant="body" color="bfc-text1" weight="medium" truncate>
						{coinMeta?.name || symbol} {isActive ? 'available' : ''}
					</Text>
					{!isActive ? (
						<Text variant="body" color="bfc-text3" weight="normal">
							{symbol}
						</Text>
=======
				<div className="max-w-token-width">
					<Text variant="body" color="gray-90" weight="semibold" truncate>
						{coinMeta?.name || symbol} {isActive ? 'available' : ''}
					</Text>
					{!isActive && !subtitle ? (
						<div className="mt-1.5">
							<Text variant="subtitle" color="steel-dark" weight="medium">
								{symbol}
							</Text>
						</div>
					) : null}
					{subtitle ? (
						<div className="mt-1.5">
							<Text variant="subtitle" color="steel" weight="medium">
								{subtitle}
							</Text>
						</div>
>>>>>>> mainnet-v1.24.1
					) : null}
				</div>

				{centerAction}

				<div className="flex flex-row justify-center items-center">
					{isActive ? (
						<Text variant="body" color="bfc-text1" weight="normal">
							{formatted}
						</Text>
					) : (
<<<<<<< HEAD
						<div data-testid={coinType} className="flex flex-col justify-end items-end">
							<Text variant="body" color="bfc-text1" weight="normal">
=======
						<div data-testid={coinType} className="max-w-token-width">
							<Text variant="body" color="gray-90" weight="medium" truncate>
>>>>>>> mainnet-v1.24.1
								{formatted} {symbol}
							</Text>
							{usd && (
								<Text variant="body" color="bfc-text3" weight="normal">
									${usd.toLocaleString('en-US')}
								</Text>
							)}
						</div>
					)}
				</div>
			</div>
		</div>
	);
}
