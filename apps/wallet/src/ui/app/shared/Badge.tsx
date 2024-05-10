// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, type VariantProps } from 'class-variance-authority';

const badgeStyle = cva(
	[
		'text-body flex uppercase font-normal px-1 py-0.5 rounded w-fit-content h-3.5 w-max justify-center items-center',
	],
	{
		variants: {
			variant: {
				warning: 'bg-bfc-orange_10p text-bfc-orange',
				success: 'bg-bfc-green_10p text-bfc-green',
			},
		},
	},
);

export interface BadgeProps extends VariantProps<typeof badgeStyle> {
	label: string;
}

export function Badge({ label, ...styles }: BadgeProps) {
	return <div className={badgeStyle(styles)}>{label}</div>;
}
