// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, type VariantProps } from 'class-variance-authority';

import type { ReactNode } from 'react';

const cardContentStyle = cva([], {
	variants: {
		variant: {
			white: 'bg-white',
			gray: 'bg-bfc-card',
		},
		padding: {
			none: 'p-0',
			small: 'p-0',
		},
		titleDivider: {
			true: 'border-t border-t-gray-45 border-solid border-0 border-transparent',
		},
	},
	defaultVariants: {
		variant: 'white',
		padding: 'small',
	},
});

export interface CardProps extends VariantProps<typeof cardContentStyle> {
	header?: ReactNode;
	footer?: ReactNode;
	children?: ReactNode;
}

export function Card({ header, footer, children, ...styleProps }: CardProps) {
	return (
		<div
			className={
				'rounded-2xl border border-solid border-gray-45 box-border overflow-hidden flex flex-col outline-1 w-full'
			}
		>
			{header && <div className="bg-gray-40 flex items-center justify-center">{header}</div>}
			<div className={cardContentStyle(styleProps)}>
				{children}
				{footer && (
					<div className={'flex flex-col pt-0 justify-center w-full'}>
						{children && <span className="h-px bg-bfc-border w-full px-2.5 mb-2.5"></span>}
						<div className="flex justify-between">{footer}</div>
					</div>
				)}
			</div>
		</div>
	);
}

export { CardItem } from './CardItem';
