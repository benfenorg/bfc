// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatAddress, isValidSuiAddress } from '@benfen/bfc.js';

import { SummaryCardFooter } from './Card';
import { Text } from '../text';
import ExplorerLink from '_src/ui/app/components/explorer-link';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { useActiveAddress } from '_src/ui/app/hooks';

export function OwnerFooter({ owner, ownerType }: { owner?: string; ownerType?: string }) {
	const address = useActiveAddress();
	const isOwner = address === owner;

	if (!owner) return null;
	const display =
		ownerType === 'Shared'
			? 'Shared'
			: isValidSuiAddress(owner)
			? isOwner
				? 'You'
				: formatAddress(owner)
			: owner;

	return (
		<SummaryCardFooter>
			<Text variant="body" weight="normal" color="bfc-text2">
				Owner
			</Text>
			<div className="flex justify-end">
				{isOwner ? (
					<Text variant="body" weight="medium" color="bfc-text1">
						{display}
					</Text>
				) : (
					<ExplorerLink
						type={ExplorerLinkType.address}
						title={owner}
						address={owner}
						className="text-bfc text-body font-medium no-underline"
					>
						{display}
					</ExplorerLink>
				)}
			</div>
		</SummaryCardFooter>
	);
}
