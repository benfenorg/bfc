// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactNode } from 'react';
import { Spinner } from './Spinner';
import classNames from 'classnames';

export function Button({
	children,
	loading,
	className,
	disabled,
	onClick,
	ghost=false,
	...props
}: {
	children: ReactNode;
	loading?: boolean;
	className?: string;
	onClick: () => Promise<void> | void;
	disabled?: boolean;
	ghost?: boolean;
}) {
	return (
		<button
			className={classNames(
				'h-10 flex w-full items-center justify-center bg-bf text-xs rounded-lg font-semibold disabled:bg-bf-disabled',
				className,
				ghost ? 'bg-white text-bf-text1 border border-bf-text1' : 'text-white'
			)}
			onClick={onClick}
			disabled={!!disabled || !!loading}
			{...props}
		>
			{loading ? <Spinner /> : children}
		</button>
	);
}
