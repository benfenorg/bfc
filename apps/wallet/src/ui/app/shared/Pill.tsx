// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, type VariantProps } from 'class-variance-authority';
import { forwardRef, type ReactNode, type Ref } from 'react';

import { ButtonOrLink, type ButtonOrLinkProps } from './utils/ButtonOrLink';

const styles = cva(
	[
		'inline-block outline-none transition no-underline bg-transparent border-none',
		'cursor-pointer',
		'truncate leading-tight uppercase text-bodySmall font-medium',
		'disabled:text-obc-text3',
	],
	{
		variants: {
			loading: {
				true: 'text-text1',
				false: '',
			},
			dark: {
				true: 'text-obc-text1 font-medium',
				false: 'text-obc-text1 font-medium',
			},
		},
		defaultVariants: {
			dark: false,
		},
	},
);

export interface PillProps
	extends Omit<VariantProps<typeof styles>, 'loading'>,
		Omit<ButtonOrLinkProps, 'className'> {
	before?: ReactNode;
	after?: ReactNode;
	text?: ReactNode;
}

export const Pill = forwardRef(
	(
		{ before, after, text, loading, dark, ...otherProps }: PillProps,
		ref: Ref<HTMLAnchorElement | HTMLButtonElement>,
	) => (
		<ButtonOrLink className={styles({ loading, dark })} {...otherProps} loading={loading} ref={ref}>
			{text}
		</ButtonOrLink>
	),
);
