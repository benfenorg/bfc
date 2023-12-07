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
	({ children, beforeIcon, afterIcon, isDarker, className, ...props }, ref) => (
		<button
			ref={ref}
			type="button"
			className={clsx(
				'flex cursor-pointer items-center gap-1 rounded-md border-none bg-transparent p-[7px] text-body font-medium outline-none',
				isDarker
					? 'text-white hover:bg-bfc-hover ui-open:bg-bfc-hover'
					: 'text-bfc-text1 hover:bg-bfc-card ui-open:bg-bfc-card',
				className,
			)}
			{...props}
		>
			{beforeIcon}
			{children}
			{afterIcon}
		</button>
	),
);
