// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { BuilderCallArg, type TransactionBlockInput } from '@benfen/bfc.js';
import { formatAddress, is, toB64 } from '@benfen/bfc.js/utils';

import ExplorerLink from '_src/ui/app/components/explorer-link';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { Text } from '_src/ui/app/shared/text';

interface InputProps {
	input: TransactionBlockInput;
}

export function Input({ input }: InputProps) {
	const { objectId } = input.value?.Object?.ImmOrOwned || input.value?.Object?.Shared || {};

	return (
		<div className="break-all">
			<Text variant="body" weight="medium" color="bfc-text2" mono>
				{is(input.value, BuilderCallArg) ? (
					'Pure' in input.value ? (
						`${toB64(new Uint8Array(input.value.Pure))}`
					) : (
						<ExplorerLink type={ExplorerLinkType.object} objectID={objectId}>
							{formatAddress(objectId)}
						</ExplorerLink>
					)
				) : (
					'Unknown input value'
				)}
			</Text>
		</div>
	);
}
