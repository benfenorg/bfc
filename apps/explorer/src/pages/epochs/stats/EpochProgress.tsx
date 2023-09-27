// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatDate } from '@mysten/core';
import { Heading, Text } from '@mysten/ui';
import clsx from 'clsx';

import { getElapsedTime, useEpochProgress } from '~/pages/epochs/utils';
import { Card } from '~/ui/Card';
import { Divider } from '~/ui/Divider';
import { ProgressBar } from '~/ui/ProgressBar';

export interface EpochProgressProps {
	epoch?: string;
	start: number;
	end?: number;
	inProgress?: boolean;
}

export function EpochProgress({ epoch, start, end, inProgress }: EpochProgressProps) {
	const { progress, label } = useEpochProgress();

	const elapsedTime = !inProgress && start && end ? getElapsedTime(start, end) : undefined;

	return (
		<Card bg={inProgress ? 'highlight' : 'default'} spacing="lg" rounded="2xl">
			<div className="flex flex-col space-y-5">
				<div className={clsx(inProgress ? 'space-y-4' : 'space-y-5')}>
					<div className="flex items-center justify-between">
						<Heading color="steel-darker" variant="heading4/semibold">
							{inProgress ? `Epoch ${epoch}` : `Epoch ${epoch}`}
						</Heading>
						{elapsedTime ? (
							<Heading variant="heading6/medium" color="steel">
								{elapsedTime}
							</Heading>
						) : (
							<Heading variant="heading6/medium" color="steel">
								in progress
							</Heading>
						)}
					</div>
					<Divider />
					<div className="space-y-1.5">
						<Text variant="body/normal" uppercase color="steel-dark">
							Start
						</Text>
						<Text variant="pHeading4/semibold" color="steel-darker">
							{formatDate(start)}
						</Text>
					</div>
					{!inProgress && end ? (
						<div>
							<Text variant="body/normal" uppercase color="steel-dark">
								End
							</Text>
							<Text variant="pHeading4/semibold" color="steel-darker">
								{formatDate(end)}
							</Text>
						</div>
					) : null}
				</div>
				{inProgress ? (
					<div className="space-y-1.5">
						<Text variant="body/normal" color="steel-dark">
							End
						</Text>
						<div className="flex items-center gap-2">
							<div className="whitespace-nowrap">
								<Text variant="pHeading4/semibold" color="steel-darker">
									{label}
								</Text>
							</div>

							<ProgressBar animate progress={progress || 0} />
						</div>
					</div>
				) : null}
			</div>
		</Card>
	);
}
