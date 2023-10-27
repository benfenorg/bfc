// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	CoinFormat,
	formatBalance,
	useGetReferenceGasPrice,
	useGetTotalTransactionBlocks,
	useGetNetworkOverview,
	useFormatCoin,
} from '@mysten/core';
import { SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import { Text } from '@mysten/ui';

import { ReactComponent as XCoinLogo } from '../../assets/XCoin.svg';
import { numberSuffix } from '../../utils/numberUtil';
import { Card } from '~/ui/Card';
import { useTokenPrice } from '~/hooks/useTokenPrice';

export function Overview() {
	const { data: referenceGasPrice } = useGetReferenceGasPrice();
	const { data: totalTransactionBlocks } = useGetTotalTransactionBlocks();
	const { data: overview } = useGetNetworkOverview();
	const { data: price } = useTokenPrice();

	const [formattedAmount] = useFormatCoin(overview?.volume24h, SUI_TYPE_ARG);

	const gasPriceFormatted =
		typeof referenceGasPrice === 'bigint'
			? formatBalance(referenceGasPrice, 0, CoinFormat.FULL)
			: null;

	return (
		<Card bg="white" spacing="lg" height="full">
			<div className="flex flex-col">
				<div>
					<div>
						<Text variant="subtitle/medium" color="gray-90">
							Token Price
						</Text>
					</div>
					<div className="mt-2.5 flex gap-4 border-b border-[#E1E1E9] pb-7.5">
						<XCoinLogo />
						<div className="flex items-baseline">
							<span className="text-[32px] font-bold text-[#171719]">{price}</span>
							<span className="px-1 text-[20px] font-bold text-[#A3A8B5]">BUSD</span>
						</div>
					</div>
				</div>
				<div className="mt-7.5 grid grid-cols-2 gap-y-7.5">
					<div>
						<div>
							<Text variant="pBody/normal" color="steel-dark">
								24H Txn Volume
							</Text>
						</div>
						<div className="mt-1.25 flex items-baseline gap-1">
							<Text variant="pHeading4/semibold" color="steel-darker">
								{formattedAmount}
							</Text>
							<Text variant="pBody/medium" color="steel-dark">
								BFC
							</Text>
						</div>
					</div>
					<div>
						<div>
							<Text variant="pBody/normal" color="steel-dark">
								24H Active Addresses
							</Text>
						</div>
						<div className="mt-1.25  items-baseline">
							<Text variant="pHeading4/semibold" color="steel-darker">
								{overview?.totalAddresses24h ? numberSuffix(Number(overview.totalAddresses24h)) : '-'}
							</Text>
						</div>
					</div>
					<div>
						<div>
							<Text variant="pBody/normal" color="steel-dark">
								Transactions
							</Text>
						</div>
						<div className="mt-1.25 flex items-baseline gap-1">
							<Text variant="pHeading4/semibold" color="steel-darker">
								{totalTransactionBlocks ? numberSuffix(Number(totalTransactionBlocks)) : '-'}
							</Text>
						</div>
					</div>
					<div>
						<div>
							<Text variant="pBody/normal" color="steel-dark">
								Gas Price
							</Text>
						</div>
						<div className="mt-1.25 flex items-baseline gap-1">
							<Text variant="pHeading4/semibold" color="steel-darker">
								{gasPriceFormatted}
							</Text>
							<Text variant="pBody/medium" color="steel-dark">
								MIST
							</Text>
						</div>
					</div>
				</div>
			</div>
		</Card>
	);
}
