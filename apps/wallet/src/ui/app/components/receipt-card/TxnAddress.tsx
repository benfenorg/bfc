// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { formatAddress } from '@benfen/bfc.js';
import { useResolveSuiNSName } from '@mysten/core';

import { Text } from '_src/ui/app/shared/text';
=======
import { Text } from '_src/ui/app/shared/text';
import { useResolveSuiNSName } from '@mysten/core';

import { TxnAddressLink } from './TxnAddressLink';
>>>>>>> mainnet-v1.24.1

type TxnAddressProps = {
	address: string;
	label: string;
};

export function TxnAddress({ address, label }: TxnAddressProps) {
	const { data: domainName } = useResolveSuiNSName(address);

	return (
		<div className="h-10 flex justify-between w-full items-center">
			<Text variant="body" weight="normal" color="bfc-text1">
				{label}
			</Text>
			<div className="flex gap-1 items-center">{domainName ?? formatAddress(address)}</div>
		</div>
	);
}
