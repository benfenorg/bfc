// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// TODO: replace all the existing button usages (the current Button component or button) with this
// TODO: rename this to Button when the existing Button component is removed

import { cva, type VariantProps } from 'class-variance-authority';
import cl from 'classnames';
import { forwardRef, type ReactNode, type Ref } from 'react';

import { ButtonOrLink, type ButtonOrLinkProps } from './utils/ButtonOrLink';

const styles = cva(
	[
		'transition no-underline outline-none group',
		'flex flex-row flex-nowrap items-center justify-center gap-2',
		'cursor-pointer max-w-full min-w-0 w-full',
	],
	{
		variants: {
			variant: {
				primary: ['primary-button'],
				secondary: ['bg-bfc-card rounded-lg border-none text-bfc-text2'],
				secondarySui: [
					'bg-transparent text-steel border-none',
					'hover:bg-sui-light focus:bg-sui-light',
					'visited:text-steel-darker',
					'active:text-steel-dark/70',
					'disabled:bg-gray-40 disabled:text-steel/50',
				],
				outline: ['bg-bfc-card text-bfc-text2 border border-solid border-bfc-border'],
				outlineWarning: [
					'bg-white border-solid border border-steel text-issue-dark',
					'hover:border-steel-dark focus:border-steel-dark',
					'visited:text-issue-dark',
					'active:border-steel active:text-issue/70',
					'disabled:border-gray-45 disabled:text-issue-dark/50',
				],
				warning: ['bg-bfc-red_10p text-bfc-red rounded-lg border-none'],
				plain: [
					'bg-transparent text-steel-darker border-none',
					'visited:text-steel-darker',
					'active:text-steel-darker/70',
					'disabled:text-steel-dark/50',
				],
				hidden: [
					'bg-gray-45 bg-opacity-25 text-gray-60 hover:text-sui-dark hover:bg-gray-35 hover:bg-opacity-75 border-none h-full w-full backdrop-blur-md',
				],
				account: [
					'flex gap-2.5 items-center justify-center bg-bfc-card border border-solid border-bfc-border rounded-lg text-bodySmall text-bfc-text1 font-medium',
				],
			},
			size: {
				tall: ['h-10 px-5 rounded-xl'],
				narrow: ['h-9 py-2.5 px-5 rounded-lg'],
				tiny: ['h-5 rounded-lg'],
				icon: ['h-full w-full rounded-lg p-1'],
			},
		},
	},
);
const iconStyles = cva('flex', {
	variants: {
		variant: {
			primary: ['text-white group-active:text-steel/70 group-disabled:text-white'],
			secondary: ['text-bfc-text2'],
			secondarySui: [
				'text-bfc',
				'group-hover:text-hero group-focus:text-hero',
				'group-active:text-hero/70',
				'group-disabled:text-hero/50',
			],
			outline: ['text-bfc-text1', '', '', ''],
			outlineWarning: [
				'text-issue-dark/80',
				'group-hover:text-issue-dark group-focus:text-issue-dark',
				'group-active:text-issue/70',
				'group-disabled:text-issue/50',
			],
			warning: [
				'text-issue-dark/80',
				'group-hover:text-issue-dark group-focus:text-issue-dark',
				'group-active:text-issue/70',
				'group-disabled:text-issue/50',
			],
			plain: [],
			hidden: [],
			account: [],
		},
	},
});

export interface ButtonProps
	extends VariantProps<typeof styles>,
		VariantProps<typeof iconStyles>,
		Omit<ButtonOrLinkProps, 'className'> {
	className?: string;
	before?: ReactNode;
	after?: ReactNode;
	text?: ReactNode;
}

export const Button = forwardRef(
	(
		{
			variant = 'primary',
			size = 'narrow',
			before,
			after,
			text,
			className,
			...otherProps
		}: ButtonProps,
		ref: Ref<HTMLAnchorElement | HTMLButtonElement>,
	) => {
		return (
			<ButtonOrLink ref={ref} className={cl(styles({ variant, size }), className)} {...otherProps}>
				{before ? <div className={iconStyles({ variant })}>{before}</div> : null}
				{text ? <div className={'truncate'}>{text}</div> : null}
				{after ? <div className={iconStyles({ variant })}>{after}</div> : null}
			</ButtonOrLink>
		);
	},
);
