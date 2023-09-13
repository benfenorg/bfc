// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, type VariantProps, cx } from 'class-variance-authority';
import clsx from 'classnames';
import { type ComponentProps, forwardRef, type ReactNode } from 'react';

const styles = cva(
	[
		'cursor-pointer outline-0 flex flex-row items-center py-1.25 px-2.5 gap-1.25 rounded-[20px] border border-solid border-obc-border',
		'hover:border-obc-text2',
		'focus:border-obc',
		'active:border-obc',
		'disabled:text-obc-text3',
	],
	{
		variants: {
			bgOnHover: {
				blueLight: ['text-obc'],
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
				<div className={cx('flex', 'text-obc')}>{iconAfter}</div>
			</button>
		);
	},
);
