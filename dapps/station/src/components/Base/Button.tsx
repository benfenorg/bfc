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
	...props
}: {
	children: ReactNode;
	loading?: boolean;
	className?: string;
	onClick: () => Promise<void> | void;
	disabled?: boolean;
}) {
	return (
		<button
			className={classNames(
				'h-10 flex w-full items-center justify-center bg-bf text-white rounded-md font-semibold',
				className,
			)}
			onClick={onClick}
			disabled={!!disabled || !!loading}
			{...props}
		>
			{loading ? <Spinner /> : children}
		</button>
	);
}
