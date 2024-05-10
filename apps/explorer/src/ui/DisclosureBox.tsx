// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ChevronRight12 } from '@mysten/icons';
import * as Collapsible from '@radix-ui/react-collapsible';
import { cva, type VariantProps } from 'class-variance-authority';
import clsx from 'clsx';
import { useState, type ReactNode, useEffect } from 'react';

const disclosureBoxStyles = cva('group', {
	variants: {
		variant: {
			primary: 'bg-bfc-card rounded-lg',
			outline:
				'bg-transparent border border-gray-45 hover:bg-bfc-card hover:border-transparent rounded-2lg',
		},
	},
	defaultVariants: {
		variant: 'primary',
	},
});

export interface DisclosureBoxProps extends VariantProps<typeof disclosureBoxStyles> {
	defaultOpen?: boolean;
	title: ReactNode;
	preview?: ReactNode;
	children: ReactNode;
	disabled?: boolean;
}

export function DisclosureBox({
	defaultOpen,
	title,
	children,
	preview,
	variant,
	disabled,
}: DisclosureBoxProps) {
	const [open, setOpen] = useState(defaultOpen);

	useEffect(() => {
		if (disabled) {
			setOpen(false);
		}
	}, [disabled]);

	return (
		<div className={disclosureBoxStyles({ variant })}>
			<Collapsible.Root open={open} onOpenChange={setOpen}>
				<Collapsible.Trigger
					className={clsx(
						'flex w-full flex-nowrap items-center gap-1 px-5 py-3.75',
						disabled ? 'cursor-not-allowed' : 'cursor-pointer ',
					)}
					disabled={disabled}
				>
					<div className="flex w-11/12 flex-1 gap-1 text-body font-semibold text-gray-90">
						<span className={clsx(disabled ? 'text-bfc-text3' : 'text-bfc')}>{title}</span>
						{preview && !open ? preview : null}
					</div>

					<ChevronRight12 className={clsx('text-caption text-steel', open && 'rotate-90')} />
				</Collapsible.Trigger>
				<Collapsible.Content className="px-5 pb-3.75">{children}</Collapsible.Content>
			</Collapsible.Root>
		</div>
	);
}
