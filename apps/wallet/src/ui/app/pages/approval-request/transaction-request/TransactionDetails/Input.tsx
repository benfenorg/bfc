// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_src/ui/app/shared/text';
import { type SerializedTransactionDataV1 } from '@benfen/bfc.js/transactions';

interface InputProps {
	input: SerializedTransactionDataV1['inputs'][number];
}

export function Input({ input }: InputProps) {
	// const { objectId } = input.Object?.ImmOrOwnedObject || input.Object?.SharedObject || {};

	return (
		<div className="break-all">
			<Text variant="pBodySmall" weight="medium" color="steel-dark" mono>
				{JSON.stringify(input)}
				{/* {'Pure' in input ? (
					`${input.Pure?.bytes}`
				) : 'Object' in input ? (
					<ExplorerLink type={ExplorerLinkType.object} objectID={objectId!}>
						{formatAddress(objectId!)}
					</ExplorerLink>
				) : (
					'Unknown input value'
				)} */}
			</Text>
		</div>
	);
}
