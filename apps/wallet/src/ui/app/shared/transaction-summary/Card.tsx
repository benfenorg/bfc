// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { cva, type VariantProps } from 'class-variance-authority';
import { type AnchorHTMLAttributes, type ElementType, type ReactNode } from 'react';

import { Heading } from '_src/ui/app/shared/heading';

const cardStyles = cva(
	['bg-white relative flex flex-col pt-5 w-full rounded-lg border border-solid border-obc-border'],
	{
		variants: {
			as: {
				div: '',
				a: 'no-underline text-hero-dark hover:text-hero visited:text-hero-dark',
			},
		},
	},
);

interface Props extends VariantProps<typeof cardStyles> {
	heading?: string;
	after?: ReactNode;
	children: ReactNode;
	footer?: ReactNode;
}

type CardProps = Props & AnchorHTMLAttributes<HTMLAnchorElement>;

export const SummaryCardFooter = ({ children }: { children: ReactNode }) => {
	return (
		<div className="h-10 px-2.5 rounded-b-lg flex justify-between items-center bg-obc-card">
			{children}
		</div>
	);
};

export function Card({ as = 'div', heading, children, after, footer = null, ...props }: CardProps) {
	const Component = as as ElementType;
	return (
		<Component className={cardStyles({ as })} {...props}>
			{heading && (
				<div className="flex items-center justify-between px-2.5">
					<Heading variant="heading4" color="obc-text1" weight="semibold">
						{heading}
					</Heading>
					{after && <div>{after}</div>}
				</div>
			)}
			{children}
			{footer}
		</Component>
	);
}
