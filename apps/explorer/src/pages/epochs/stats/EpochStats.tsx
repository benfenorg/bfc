// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Heading } from '@mysten/ui';
import { type ReactNode } from 'react';

import { Card } from '~/ui/Card';
import { Divider } from '~/ui/Divider';

interface EpochStatsProps {
	label: string;
	children: ReactNode;
}

export function EpochStats({ label, children }: EpochStatsProps) {
	return (
		<Card spacing="lg" rounded="2xl">
			<div className="flex flex-col gap-5">
				{label && (
					<Heading color="steel-darker" variant="heading4/semibold">
						{label}
					</Heading>
				)}
				<Divider />
				<div className="grid grid-cols-2 gap-5">{children}</div>
			</div>
		</Card>
	);
}
