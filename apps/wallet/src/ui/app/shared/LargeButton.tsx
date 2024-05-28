// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import LoadingIndicator from '_components/loading/LoadingIndicator';
import clsx from 'clsx';
import { forwardRef, type ReactNode, type Ref } from 'react';

import { ButtonOrLink, type ButtonOrLinkProps } from './utils/ButtonOrLink';

function Decorator({ disabled, children }: { disabled?: boolean; children: ReactNode }) {
	return (
		<div
			className={clsx(
				'text-heading2 bg-transparent text-center',
				disabled ? 'text-gray-60' : 'text-hero-dark group-hover:text-hero',
			)}
		>
			{children}
		</div>
	);
}

interface LargeButtonProps extends ButtonOrLinkProps {
	children: ReactNode;
	loading?: boolean;
	before?: ReactNode;
	after?: ReactNode;
	top?: ReactNode;
	center?: boolean;
	disabled?: boolean;
	primary?: boolean;
}

export const LargeButton = forwardRef(
	(
<<<<<<< HEAD
		{ top, before, after, center, loading, disabled, children, ...otherProps }: LargeButtonProps,
=======
		{
			top,
			before,
			after,
			center,
			spacing,
			loading,
			disabled,
			children,
			primary,
			className,
			...otherProps
		}: LargeButtonProps,
>>>>>>> mainnet-v1.24.1
		ref: Ref<HTMLAnchorElement | HTMLButtonElement>,
	) => {
		return (
			<ButtonOrLink
				ref={ref}
				{...otherProps}
				className={clsx(
<<<<<<< HEAD
					'group border border-solid border-transparent flex rounded-2xl items-center w-full p-3.75 justify-between no-underline',
					disabled ? 'bg-gray-40' : 'bg-sui/10 hover:shadow-drop hover:border-sui/10',
=======
					'group border border-solid border-transparent flex rounded-md items-center py-2 px-8 justify-between no-underline',
					disabled ? 'bg-hero-darkest/5 pointer-events-none' : 'bg-white/80 hover:border-sui/10',
					primary ? '!bg-sui-primaryBlue2023' : '',
					spacing === 'sm' && '!p-3',
					className,
>>>>>>> mainnet-v1.24.1
				)}
			>
				{loading && (
					<div className="p-2 w-full flex items-center h-full">
						<LoadingIndicator />
					</div>
				)}
				{!loading && (
					<div className={clsx('flex items-center w-full gap-2.5', center && 'justify-center')}>
						{before && <Decorator disabled={disabled}>{before}</Decorator>}
						<div className="flex flex-col">
							{top && <Decorator disabled={disabled}>{top}</Decorator>}
							<div
								className={clsx(
<<<<<<< HEAD
									'text-body font-semibold',
									disabled ? 'text-gray-60' : 'text-hero-dark group-hover:text-hero',
=======
									'text-bodySmall font-semibold',
									disabled ? 'text-steel' : 'text-hero-dark group-hover:text-hero',
									primary ? '!text-white' : '',
>>>>>>> mainnet-v1.24.1
								)}
							>
								{children}
							</div>
						</div>
						{after && (
							<div className="ml-auto">
								<Decorator disabled={disabled}>{after}</Decorator>
							</div>
						)}
					</div>
				)}
			</ButtonOrLink>
		);
	},
);
