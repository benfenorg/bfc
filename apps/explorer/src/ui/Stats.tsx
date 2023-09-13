// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Heading, Text } from '@mysten/ui';
import clsx from 'clsx';

import { ReactComponent as InfoSvg } from './icons/info_14x14.svg';
import { Tooltip } from '~/ui/Tooltip';
import { ampli } from '~/utils/analytics/ampli';

import type { ReactNode } from 'react';

export type StatsProps = {
	size?: 'sm' | 'md';
	label: string;
	children?: ReactNode;
	tooltip?: string;
	unavailable?: boolean;
	postfix?: ReactNode;
	darker?:boolean;
	orientation?: 'horizontal' | 'vertical';
	color?: 'steel-darker' | 'hero' | 'steel-dark';
};

export function Stats({
	label,
	children,
	tooltip,
	unavailable,
	postfix,
	darker,
	size = 'md',
	orientation = 'vertical',
	color = 'steel-dark',
}: StatsProps) {
	return (
		<div
			className={clsx(
				'flex max-w-full flex-nowrap justify-between gap-1.5',
				orientation === 'horizontal' ? '' : 'flex-col',
			)}
		>
			<div className="flex items-center justify-start gap-1 overflow-hidden text-caption">
				<Text variant="pBody/normal" color={color} truncate>
					{label}
				</Text>
				{tooltip && (
					<Tooltip
						tip={unavailable ? 'Coming soon' : tooltip}
						onOpen={() => {
							ampli.activatedTooltip({ tooltipLabel: label });
						}}
					>
						<InfoSvg />
					</Tooltip>
				)}
			</div>
			<div className="flex items-baseline gap-0.5">
				<Heading
					variant={size === 'md' ? 'heading4/semibold' : 'heading6/semibold'}
					color={unavailable || darker ? 'steel-darker' : color}
				>
					{unavailable || children == null ? '--' : children}
				</Heading>

				{postfix && (
					<Text
						// variant={size === 'md' ? 'heading3/semibold' : 'heading6/semibold'}
						// color={unavailable ? 'steel-darker' : color}
						variant="body/normal"
						color="steel-dark"
					>
						{postfix}
					</Text>
				)}
			</div>
		</div>
	);
}
