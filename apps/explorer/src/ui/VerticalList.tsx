// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import clsx from 'clsx';
import { type ReactNode } from 'react';

export interface ListItemProps {
	active?: boolean;
	children: ReactNode;
	type?: string;
	onClick?(): void;
}

export function ListItem({ active, children, type, onClick }: ListItemProps) {
	const rounded: string = type === 'borderLine' ? 'rounded-r-md border-l-2' : 'rounded-md border';
	const border: string = type === 'borderLine' ? 'border-bfc-text2' : 'border-bfc-card';
	return (
		<li className="list-none">
			<button
				type="button"
				className={clsx(
					`block w-full cursor-pointer ${rounded} px-2.5 py-2 text-left text-body`,
					active
						? `${border} bg-bfc-card font-semibold text-bfc-text1`
						: 'border-transparent bg-white font-medium text-bfc-text2',
				)}
				onClick={onClick}
			>
				{children}
			</button>
		</li>
	);
}

export interface VerticalListProps {
	children: ReactNode;
}

export function VerticalList({ children }: VerticalListProps) {
	return <ul className="m-0 flex flex-col gap-1 p-0">{children}</ul>;
}
