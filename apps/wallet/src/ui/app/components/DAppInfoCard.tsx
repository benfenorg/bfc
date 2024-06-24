// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getValidDAppUrl } from '_src/shared/utils';
import { ArrowUpRight12 } from '@mysten/icons';
import { cx } from 'class-variance-authority';

import { Heading } from '../shared/heading';
import { Link } from '../shared/Link';
import { Text } from '../shared/text';
import { AccountAddress } from './AccountAddress';

export type DAppInfoCardProps = {
	name: string;
	url: string;
	iconUrl?: string;
	connectedAddress?: string;
};

export function DAppInfoCard({ name, url, iconUrl, connectedAddress }: DAppInfoCardProps) {
	const validDAppUrl = getValidDAppUrl(url);
	const appHostname = validDAppUrl?.hostname ?? url;

	return (
		<div className="flex flex-col items-stretch p-2.5 pt-5 rounded-lg border border-solid border-bfc-border">
			<div className="flex flex-row flex-nowrap items-center gap-2.5 mb-2.5">
				<div className="flex items-stretch h-12 w-12 rounded-full overflow-hidden bg-[#d9d9d9] shrink-0 grow-0">
					{iconUrl ? <img className="flex-1" src={iconUrl} alt={name} /> : null}
				</div>
				<div className="flex flex-col flex-nowrap gap-1.5">
					<Heading variant="heading4" weight="semibold" color="bfc-text1">
						{name}
					</Heading>
					<Text variant="body" weight="normal" color="bfc-text3">
						{appHostname}
					</Text>
				</div>
			</div>
			{connectedAddress ? (
				<div className="p-2.5 flex flex-nowrap flex-row items-center gap-1.25">
					<Text variant="body" weight="medium" color="bfc-text1" truncate>
						Connected account
					</Text>
					<div className="flex-1" />
					<AccountAddress copyable address={connectedAddress} />
				</div>
			) : null}
			<div
				className={cx(
					'p-2.5 pb-0 flex justify-start border-0 border-t border-solid border-bfc-border',
				)}
			>
				<Link
					href={validDAppUrl?.toString() ?? url}
					title={name}
					size="body"
					text="View Website"
					after={<ArrowUpRight12 />}
					color="bfc-link"
					weight="medium"
				/>
			</div>
		</div>
	);
}
