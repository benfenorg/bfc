// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Disclosure } from '@headlessui/react';

import { Heading } from '../heading';

import * as CollapsiblePrimitive from '@radix-ui/react-collapsible';
import cn from 'classnames';
import { useState, type ReactNode } from 'react';

type CollapseProps = {
	title: string;
	initialIsOpen?: boolean;
	children: ReactNode | ReactNode[];
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
									<Heading variant="heading4" weight="semibold" color="obc-text1">
										{title}
									</Heading>
								</div>
								<div className="h-px bg-obc-border w-full" />
							</div>
						</Disclosure.Button>

						<Disclosure.Panel>
							<div className="pt-2.5">{children}</div>
						</Disclosure.Panel>
					</>
				)}
			</Disclosure>
		</div>
	);
}
