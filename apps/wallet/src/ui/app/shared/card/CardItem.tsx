// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import type { ReactNode } from 'react';

export type CardItemProps = {
	title: ReactNode;
	children: ReactNode;
};

export function CardItem({ title, children }: CardItemProps) {
	return (
		<div
			className={
				'p-2.5 flex flex-col flex-nowrap max-w-full gap-1.25 flex-1 justify-center items-center'
			}
		>
			<Text variant="body" weight="normal" color="bfc-text2">
				{title}
			</Text>

			{children}
		</div>
	);
}
