// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { type GasSummaryType, useFormatCoin } from '@mysten/core';
import { formatAddress } from '@mysten/sui.js/utils';

import { Heading } from '../../heading';
import { Text } from '../../text';
import ExplorerLink from '_src/ui/app/components/explorer-link';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { useActiveAddress } from '_src/ui/app/hooks';
import { GAS_TYPE_ARG } from '_src/ui/app/redux/slices/sui-objects/Coin';

export function GasSummary({ gasSummary }: { gasSummary?: GasSummaryType }) {
	const [gas, symbol] = useFormatCoin(gasSummary?.totalGas, GAS_TYPE_ARG);
	const address = useActiveAddress();

	if (!gasSummary) return null;

	return (
		<div className="flex flex-col justify-stretch rounded-lg border border-solid border-obc-border">
			<div className="h-10 px-2.5 bg-obc-card flex items-center rounded-t-lg">
				<Heading variant="heading4" color="obc-text1" weight="semibold">
					Gas Fees
				</Heading>
			</div>
			<div className="p-2.5 flex flex-col items-center gap-2.5 w-full">
				<div className="flex w-full items-center justify-start">
					{address === gasSummary?.owner && (
						<div className="mr-auto">
							<Text color="obc-text2" variant="body" weight="normal">
								You Paid
							</Text>
						</div>
					)}
					<Text color="obc-text1" variant="body" weight="medium">
						{gasSummary?.isSponsored ? '0' : gas} {symbol}
					</Text>
				</div>
				{gasSummary?.isSponsored && gasSummary.owner && (
					<>
						<div className="flex w-full justify-between">
							<Text color="obc-text2" variant="body" weight="normal">
								Paid by Sponsor
							</Text>
							<Text color="obc-text1" variant="body" weight="medium">
								{gas} {symbol}
							</Text>
						</div>
						<div className="flex w-full justify-between">
							<Text color="obc-text2" variant="body" weight="normal">
								Sponsor
							</Text>
							<ExplorerLink
								type={ExplorerLinkType.address}
								address={gasSummary.owner}
								className="no-underline"
							>
								<Text variant="body" color="obc-text1" weight="medium" truncate>
									{formatAddress(gasSummary.owner)}
								</Text>
							</ExplorerLink>
						</div>
					</>
				)}
			</div>
		</div>
	);
}
