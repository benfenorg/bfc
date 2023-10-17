// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	CoinFormat,
	formatBalance,
	useGetReferenceGasPrice,
	useGetTotalTransactionBlocks,
	useGetNetworkOverview,
} from '@mysten/core';
import { Text } from '@mysten/ui';

import { ReactComponent as XCoinLogo } from '../../assets/XCoin.svg';
import { numberSuffix } from '../../utils/numberUtil';
import { Card } from '~/ui/Card';

export function Overview() {
	const { data: referenceGasPrice } = useGetReferenceGasPrice();
	const { data: totalTransactionBlocks } = useGetTotalTransactionBlocks();
	const { data: overview } = useGetNetworkOverview();

	console.log('overview', overview);
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
						<div>
							<span className="text-[32px] font-bold text-[#171719]">12.89234</span>
							<span className="px-1 text-[20px] font-bold text-[#A3A8B5]">OST</span>
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
								12332
							</Text>
							<Text variant="pBody/medium" color="steel-dark">
								BFC
							</Text>
						</div>
					</div>
					<div>
						<div>
							<Text variant="subtitle/medium" color="gray-90">
								24H Active Addresses
							</Text>
						</div>
						<div className="mt-1.25">
							<Text variant="pHeading4/semibold" color="steel-darker">
								12332
							</Text>
						</div>
					</div>
					<div>
						<div>
							<Text variant="subtitle/medium" color="gray-90">
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
							<Text variant="subtitle/medium" color="gray-90">
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
