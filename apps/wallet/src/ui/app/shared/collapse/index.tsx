// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { Disclosure } from '@headlessui/react';

import { type ReactNode } from 'react';
import { Heading } from '../heading';
=======
import { ChevronDown12, ChevronRight12 } from '@mysten/icons';
import * as CollapsiblePrimitive from '@radix-ui/react-collapsible';
import cn from 'clsx';
import { useState, type ReactNode } from 'react';
>>>>>>> mainnet-v1.24.1

type CollapseProps = {
	title: string;
	initialIsOpen?: boolean;
	children: ReactNode | ReactNode[];
<<<<<<< HEAD
};

export function Collapse({ title, children, initialIsOpen = false }: CollapseProps) {
	return (
		<div>
			<Disclosure defaultOpen={initialIsOpen}>
				{({ open }) => (
					<>
						<Disclosure.Button as="div" className="flex w-full flex-col gap-2 cursor-pointer">
							<div className="flex items-center gap-1">
								<div className="w-full">
									<Heading variant="heading4" weight="semibold" color="bfc-text1">
										{title}
									</Heading>
								</div>
								<div className="h-px bg-bfc-border w-full" />
							</div>
						</Disclosure.Button>
=======
	shade?: 'lighter' | 'darker';
	isOpen?: boolean;
	onOpenChange?: (isOpen: boolean) => void;
}

export function Collapsible({
	title,
	children,
	defaultOpen,
	isOpen,
	onOpenChange,
	shade = 'lighter',
}: CollapsibleProps) {
	const [open, setOpen] = useState(isOpen ?? defaultOpen ?? false);

	const handleOpenChange = (isOpen: boolean) => {
		setOpen(isOpen);
		onOpenChange?.(isOpen);
	};

	return (
		<CollapsiblePrimitive.Root
			className="flex flex-shrink-0 justify-start flex-col w-full gap-3"
			open={isOpen ?? open}
			onOpenChange={handleOpenChange}
		>
			<CollapsiblePrimitive.Trigger className="flex items-center gap-2 w-full bg-transparent border-none p-0 cursor-pointer group">
				<div
					className={cn('text-captionSmall font-semibold uppercase group-hover:text-hero', {
						'text-steel': shade === 'lighter',
						'text-steel-darker': shade === 'darker',
					})}
				>
					{title}
				</div>
				<div
					className={cn('h-px group-hover:bg-hero flex-1', {
						'bg-steel': shade === 'darker',
						'bg-gray-45 group-hover:bg-steel': shade === 'lighter',
					})}
				/>
				<div
					className={cn('group-hover:text-hero inline-flex', {
						'text-steel': shade === 'darker',
						'text-gray-45': shade === 'lighter',
					})}
				>
					{open ? <ChevronDown12 /> : <ChevronRight12 />}
				</div>
			</CollapsiblePrimitive.Trigger>
>>>>>>> mainnet-v1.24.1

						<Disclosure.Panel>
							<div className="pt-2.5">{children}</div>
						</Disclosure.Panel>
					</>
				)}
			</Disclosure>
		</div>
	);
}
