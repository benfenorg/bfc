// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import clsx from 'classnames';
import type { ReactNode } from 'react';

import { Text } from '../shared/text';

export type SummaryCardProps = {
	header?: string;
	body: ReactNode;
	footer?: ReactNode;
	minimalPadding?: boolean;
	showDivider?: boolean;
	noBorder?: boolean;
	boxShadow?: boolean;
};

export function SummaryCard({
	body,
	header,
	footer,
	minimalPadding,
	showDivider = false,
	noBorder = false,
	boxShadow = false,
}: SummaryCardProps) {
	return (
		<div
			className={clsx(
				{ 'border border-solid border-bfc-border': !noBorder, 'shadow-summary-card': boxShadow },
				'bg-white flex flex-col flex-nowrap rounded-lg w-full',
			)}
		>
			{header ? (
				<div className="flex flex-row flex-nowrap items-center justify-center uppercase bg-bfc-card p-2.5 rounded-t-lg">
					<Text variant="body" weight="semibold" color="bfc-text1" truncate>
						{header}
					</Text>
				</div>
			) : null}
			<div
				className={clsx(
					'flex-1 flex flex-col items-stretch flex-nowrap px-2.5 overflow-y-auto',
					showDivider ? 'divide-x-0 divide-y divide-bfc-border divide-solid' : '',
				)}
			>
				{body}
			</div>
			{footer ? (
				<div className="p-2.5 border-x-0 border-b-0 border-t border-solid border-bfc-border">
					{footer}
				</div>
			) : null}
		</div>
	);
}
