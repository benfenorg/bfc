// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import clsx from 'clsx';
import { forwardRef, type ComponentProps, type ReactNode } from 'react';

export interface NavItemProps extends ComponentProps<'button'> {
	beforeIcon?: ReactNode;
	afterIcon?: ReactNode;
	children: ReactNode;
	isDarker?: boolean;
}

export const NavItem = forwardRef<HTMLButtonElement, NavItemProps>(
	({ children, beforeIcon, afterIcon, isDarker, ...props }, ref) => (
		<button
			ref={ref}
			type="button"
			className={clsx(
				'flex cursor-pointer items-center gap-1 rounded-md border-none bg-transparent px-3.5 py-2 text-body font-medium outline-none',
				isDarker
					? 'text-white hover:bg-obc-hover ui-open:bg-obc-hover'
					: 'text-obc-text1 hover:bg-obc-card ui-open:bg-obc-card',
			)}
			{...props}
		>
			{beforeIcon}
			{children}
			{afterIcon}
		</button>
	),
);
