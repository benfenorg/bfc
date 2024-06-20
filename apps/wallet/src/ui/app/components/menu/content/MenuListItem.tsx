// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ChevronRight16 } from '@mysten/icons';
import type { MouseEventHandler, ReactNode } from 'react';
import { Link } from 'react-router-dom';

export type ItemProps = {
	icon: ReactNode;
	title: ReactNode;
	subtitle?: ReactNode;
	iconAfter?: ReactNode;
	to?: string;
	href?: string;
	onClick?: MouseEventHandler<Element>;
};

function MenuListItem({
	icon,
	title,
	subtitle,
	iconAfter,
	to = '',
	href = '',
	onClick,
}: ItemProps) {
	const Component = to ? Link : 'div';

	const MenuItemContent = (
		<>
			<div className="flex flex-nowrap flex-1 items-center overflow-hidden basis-3/5">
				<div className="w-6 h-6 flex text-bfc flex-none">{icon}</div>
				<div className="ml-1.25 flex-1 text-bfc text-bodySmall font-medium truncate">{title}</div>
			</div>
			<div className="flex flex-nowrap flex-1 justify-end gap-1 items-center overflow-hidden basis-2/5">
				{subtitle ? (
					<div className="transition truncate text-bfc-text2 text-bodySmall font-medium group-hover:text-steel-darker">
						{subtitle}
					</div>
				) : null}
				<div className="transition flex text-bfc-text2 flex-none text-base">
					{iconAfter || (to && <ChevronRight16 />) || null}
				</div>
			</div>
		</>
	);

	if (href) {
		return (
			<a
				href={href}
				target="_blank"
				rel="noreferrer noopener"
				className="flex flex-nowrap items-center no-underline overflow-hidden group cursor-pointer"
			>
				{MenuItemContent}
			</a>
		);
	}
	return (
		<Component
			data-testid={title}
			className="flex flex-nowrap items-center no-underline overflow-hidden group cursor-pointer"
			to={to}
			onClick={onClick}
		>
			{MenuItemContent}
		</Component>
	);
}

export default MenuListItem;
