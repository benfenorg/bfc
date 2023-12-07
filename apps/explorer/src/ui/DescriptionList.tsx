// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, type VariantProps } from 'class-variance-authority';

import type { ReactNode } from 'react';

const descriptionItemStyles = cva(['flex flex-col gap-2'], {
	variants: {
		align: {
			start: 'md:items-start',
			center: 'md:items-center',
		},
		direction: {
			default: 'md:flex-row md:gap-10',
			cloumn: '',
		},
	},

	defaultVariants: {
		align: 'center',
		direction: 'default',
	},
});

const descriptionItemLabelStyles = cva(
	['w-full flex-shrink-0 text-pBody font-medium text-steel-dark'],
	{
		variants: {
			labelWidth: {
				md: 'md:w-40',
				sm: 'md:w-28',
			},
		},
		defaultVariants: {
			labelWidth: 'md',
		},
	},
);

type DescriptionItemStylesProps = VariantProps<typeof descriptionItemStyles>;
type DescriptionItemLabelStylesProps = VariantProps<typeof descriptionItemLabelStyles>;

export interface DescriptionItemProps
	extends DescriptionItemStylesProps,
		DescriptionItemLabelStylesProps {
	title: string | ReactNode;
	children: ReactNode;
}

export function DescriptionItem({
	title,
	align,
	labelWidth,
	direction,
	children,
}: DescriptionItemProps) {
	return (
		<div className={descriptionItemStyles({ align, direction })}>
			<dt className={descriptionItemLabelStyles({ labelWidth })}>{title}</dt>
			<dd className="ml-0 min-w-0 max-w-full flex-1 leading-none">{children}</dd>
		</div>
	);
}

export type DescriptionListProps = {
	children: ReactNode;
};

export function DescriptionList({ children }: DescriptionListProps) {
	return <dl className="mt-4 flex flex-col gap-4">{children}</dl>;
}

export function DescriptionFlexList({ children }: DescriptionListProps) {
	return <dl className="mt-4 flex gap-4">{children}</dl>;
}
