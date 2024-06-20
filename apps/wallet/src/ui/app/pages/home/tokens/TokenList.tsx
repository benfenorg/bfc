// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Disclosure } from '@headlessui/react';
import { ChevronDown12, ChevronRight12 } from '@mysten/icons';
import { type ReactNode } from 'react';

type Props = {
	title: string;
	defaultOpen?: boolean;
	children: ReactNode;
};

export function TokenList({ title, defaultOpen, children }: Props) {
	return (
		<div className="flex flex-shrink-0 justify-start flex-col w-full mt-5">
			<Disclosure defaultOpen={defaultOpen}>
				{({ open }) => (
					<div className="w-full flex flex-col justify-start gap-2">
						<Disclosure.Button className="flex items-center gap-2 w-full bg-transparent border-none p-0 cursor-pointer group">
							<div className="text-body font-normal uppercase text-bfc-text2">{title}</div>
							<div className="h-px bg-bfc-text2  group-hover:bg-bfc flex-1" />
							<div className="text-bfc-text2 group-hover:text-bfc inline-flex">
								{open ? <ChevronDown12 /> : <ChevronRight12 />}
							</div>
						</Disclosure.Button>

						<Disclosure.Panel>
							<div className="flex flex-col w-full justify-center">{children}</div>
						</Disclosure.Panel>
					</div>
				)}
			</Disclosure>
		</div>
	);
}
