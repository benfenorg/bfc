// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import { CoinIcon } from '_components/coin-icon';
import { useFormatCoin } from '@mysten/core';
import cl from 'classnames';
import { type ReactNode } from 'react';

type CoinItemProps = {
	coinType: string;
	balance: bigint;
	isActive?: boolean;
	usd?: number;
	centerAction?: ReactNode;
};

export function CoinItem({ coinType, balance, isActive, usd, centerAction }: CoinItemProps) {
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
				<div className="flex flex-col">
					<Text variant="body" color="bfc-text1" weight="medium" truncate>
						{coinMeta?.name || symbol} {isActive ? 'available' : ''}
					</Text>
					{!isActive ? (
						<Text variant="body" color="bfc-text3" weight="normal">
							{symbol}
						</Text>
					) : null}
				</div>

				{centerAction}

				<div className="flex flex-row justify-center items-center">
					{isActive ? (
						<Text variant="body" color="bfc-text1" weight="normal">
							{formatted}
						</Text>
					) : (
						<div data-testid={coinType} className="flex flex-col justify-end items-end">
							<Text variant="body" color="bfc-text1" weight="normal">
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
