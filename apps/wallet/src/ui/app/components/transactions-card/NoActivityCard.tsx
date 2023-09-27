// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { TransferObject16 } from '@mysten/icons';

import { Text } from '_src/ui/app/shared/text';

export type NoActivityCardType = {
	message: string;
};

export function NoActivityCard({ message }: NoActivityCardType) {
	return (
		<div className="flex flex-col gap-4 justify-center items-center text-center h-full px-10">
			<TransferObject16 className="text-obc-text2 text-3xl" />
			<Text variant="body" weight="medium" color="obc-text2">
				{message}
			</Text>
		</div>
	);
}
