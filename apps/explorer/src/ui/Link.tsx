// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { cva, type VariantProps } from 'class-variance-authority';
import { type ReactNode } from 'react';

import { ButtonOrLink, type ButtonOrLinkProps } from './utils/ButtonOrLink';

const linkStyles = cva([], {
	variants: {
		variant: {
			text: 'text-body font-semibold text-steel-dark hover:text-steel-darker active:text-steel disabled:text-gray-60',
			mono: 'font-mono text-body font-medium text-bfc-link break-all',
			textHeroDark: 'text-pBody font-medium text-hero-dark hover:text-hero-darkest',
			large: 'font-mono text-heading4 font-bold text-bfc-link break-all',
			content: '',
		},
		uppercase: {
			true: 'uppercase',
		},
		size: {
			md: '!text-body',
			sm: '!text-bodySmall',
			captionSmall: '!text-captionSmall',
		},
	},
	defaultVariants: {
		variant: 'text',
	},
});

const linkContentStyles = cva(['flex-nowrap items-center hover:underline'], {
	variants: {
		gap: {
			'gap-1': 'gap-1',
			'gap-2': 'gap-2',
		},
		display: {
			'inline-flex': 'inline-flex',
			block: 'block',
			flex: 'flex',
		},
		underline: {
			underline: 'hover:underline',
			'no-underline': 'hover:no-underline',
		},
	},
	defaultVariants: {
		gap: 'gap-2',
		display: 'flex',
		underline: 'underline',
	},
});

type LinkContentStylesProps = VariantProps<typeof linkContentStyles>;

export interface LinkProps
	extends ButtonOrLinkProps,
		VariantProps<typeof linkStyles>,
		LinkContentStylesProps {
	before?: ReactNode;
	after?: ReactNode;
}

export function Link({
	variant,
	uppercase,
	size,
	before,
	after,
	children,
	display,
	gap,
	...props
}: LinkProps) {
	return (
		<ButtonOrLink className={linkStyles({ variant, size, uppercase })} {...props}>
			<div
				className={linkContentStyles({
					gap,
					display,
					underline: variant === 'mono' ? 'underline' : 'no-underline',
				})}
			>
				{before}
				{children}
				{after}
			</div>
		</ButtonOrLink>
	);
}
