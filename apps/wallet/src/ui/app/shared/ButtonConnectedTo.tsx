// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, cx, type VariantProps } from 'class-variance-authority';
import clsx from 'classnames';
import { forwardRef, type ComponentProps, type ReactNode } from 'react';

const styles = cva(
	[
		'cursor-pointer outline-0 flex flex-row items-center py-1.25 px-2.5 gap-1.25 rounded-[20px] border border-solid border-bfc-border',
		'hover:border-bfc-text2',
		'focus:border-bfc',
		'active:border-bfc',
		'disabled:text-bfc-text3',
	],
	{
		variants: {
			bgOnHover: {
				blueLight: ['text-bfc'],
			},
		},
		defaultVariants: {
			bgOnHover: 'blueLight',
		},
	},
);

export interface ButtonConnectedToProps
	extends VariantProps<typeof styles>,
		Omit<ComponentProps<'button'>, 'ref' | 'className'> {
	iconBefore?: ReactNode;
	text?: ReactNode;
	iconAfter?: ReactNode;
	truncate?: boolean;
}

export const ButtonConnectedTo = forwardRef<HTMLButtonElement, ButtonConnectedToProps>(
	({ bgOnHover, iconBefore, iconAfter, text, truncate, ...rest }, ref) => {
		return (
			<button {...rest} ref={ref} className={styles({ bgOnHover })}>
				<div className="flex">{iconBefore}</div>
				<div className={clsx('overflow-hidden', truncate && 'truncate')}>{text}</div>
				<div className={cx('flex', 'text-bfc')}>{iconAfter}</div>
			</button>
		);
	},
);
