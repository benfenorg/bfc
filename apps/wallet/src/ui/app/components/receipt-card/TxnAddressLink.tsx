// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import { formatAddress } from '@benfen/bfc.js/utils';
import { isSuiNSName } from '@mysten/core';

type TxnAddressLinkProps = {
	address: string;
};

export function TxnAddressLink({ address }: TxnAddressLinkProps) {
	return (
		<ExplorerLink
			type={ExplorerLinkType.address}
			address={address}
			title="View on BenFen Explorer"
			showIcon={false}
		>
			{isSuiNSName(address) ? address : formatAddress(address)}
		</ExplorerLink>
	);
}
