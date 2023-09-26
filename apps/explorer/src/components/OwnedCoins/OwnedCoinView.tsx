// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';
<<<<<<< HEAD
import { ArrowShowAndHideRight12 } from '@mysten/icons';
import { type CoinBalance } from '@mysten/sui.js';
=======
import { ArrowShowAndHideRight12, Warning16 } from '@mysten/icons';
import { SUI_TYPE_ARG } from '@mysten/sui.js/utils';
>>>>>>> heads/mainnet-v1.9.1
import { Text } from '@mysten/ui';
import * as Collapsible from '@radix-ui/react-collapsible';
import clsx from 'clsx';
import { useState } from 'react';

import { type CoinBalanceVerified } from '.';
import { CoinIcon } from './CoinIcon';
import CoinsPanel from './OwnedCoinsPanel';
<<<<<<< HEAD

type OwnedCoinViewProps = {
	coin: CoinBalance;
=======
import { Banner } from '~/ui/Banner';
import { Tooltip } from '~/ui/Tooltip';
import { ampli } from '~/utils/analytics/ampli';

type OwnedCoinViewProps = {
	coin: CoinBalanceVerified;
>>>>>>> heads/mainnet-v1.9.1
	id: string;
};

export default function OwnedCoinView({ coin, id }: OwnedCoinViewProps) {
<<<<<<< HEAD
	const [open, setOpen] = useState(false);
=======
	const isSuiCoin = coin.coinType === SUI_TYPE_ARG;
	const [open, setOpen] = useState(isSuiCoin);
>>>>>>> heads/mainnet-v1.9.1
	const [formattedTotalBalance, symbol] = useFormatCoin(coin.totalBalance, coin.coinType);

	return (
		<Collapsible.Root open={open} onOpenChange={setOpen}>
			<Collapsible.Trigger
				data-testid="ownedcoinlabel"
<<<<<<< HEAD
				className="grid w-full grid-cols-3 items-center justify-between rounded-none py-2 text-left hover:bg-sui-light"
			>
				<div className="flex">
					<ArrowShowAndHideRight12
						className={clsx('mr-1.5 text-gray-60', open && 'rotate-90 transform')}
					/>
					<Text color="steel-darker" variant="body/medium">
						{symbol}
					</Text>
				</div>

				<Text color="steel-darker" variant="body/medium">
					{coin.coinObjectCount}
				</Text>

				<div className="flex items-center gap-1">
					<Text color="steel-darker" variant="bodySmall/medium">
						{formattedTotalBalance}
					</Text>
					<Text color="steel" variant="subtitleSmallExtra/normal">
						{symbol}
					</Text>
				</div>
			</Collapsible.Trigger>

			<Collapsible.Content>
				<div className="flex flex-col gap-1 bg-gray-40 p-3">
=======
				className={clsx(
					'mt-1 flex w-full items-center rounded-lg bg-opacity-5 p-2 text-left hover:bg-hero-darkest hover:bg-opacity-5',
					open ? 'rounded-b-none bg-hero-darkest pt-3' : 'rounded-b-lg',
				)}
			>
				<div className="flex w-[45%] items-center gap-1 truncate">
					<ArrowShowAndHideRight12
						width={12}
						className={clsx('text-gray-60', open && 'rotate-90 transform')}
					/>

					<div className="flex items-center gap-3 truncate">
						<div className="w-6">
							<CoinIcon coinType={coin.coinType} size="sm" />
						</div>
						<Text color="steel-darker" variant="body/medium" truncate>
							{symbol}
						</Text>
					</div>

					{!coin.isRecognized && (
						<Tooltip
							tip="This coin has not been recognized by Sui Foundation."
							onOpen={() =>
								ampli.activatedTooltip({
									tooltipLabel: 'unrecognizedCoinWarning',
								})
							}
						>
							<Banner variant="warning" icon={null} border spacing="sm">
								<Warning16 />
							</Banner>
						</Tooltip>
					)}
				</div>

				<div className="flex w-[25%] pl-2">
					<Text color={coin.isRecognized ? 'steel-darker' : 'gray-60'} variant="body/medium">
						{coin.coinObjectCount}
					</Text>
				</div>

				<div className="flex w-[30%] items-center gap-1 truncate pl-1">
					<Text
						color={coin.isRecognized ? 'steel-darker' : 'gray-60'}
						variant="bodySmall/medium"
						truncate
					>
						{formattedTotalBalance}
					</Text>
					<Text color="steel" variant="subtitleSmallExtra/normal" truncate>
						{symbol}
					</Text>
				</div>
			</Collapsible.Trigger>

			<Collapsible.Content>
				<div className="flex flex-col gap-1 rounded-bl-lg rounded-br-lg bg-gray-40 p-3">
>>>>>>> heads/mainnet-v1.9.1
					<CoinsPanel id={id} coinType={coin.coinType} />
				</div>
			</Collapsible.Content>
		</Collapsible.Root>
	);
}
