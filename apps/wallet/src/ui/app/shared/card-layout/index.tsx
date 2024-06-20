// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Heading } from '_app/shared/heading';
import { Text } from '_app/shared/text';
import { Handclap, Logo } from '@mysten/icons';
import type { ReactNode } from 'react';

export type CardLayoutProps = {
	title?: string;
	subtitle?: string;
	headerCaption?: string;
	icon?: 'success' | 'bfc';
	children: ReactNode | ReactNode[];
};

export function CardLayout({ children, title, subtitle, headerCaption, icon }: CardLayoutProps) {
	return (
		<div className="flex flex-col flex-nowrap rounded-xl items-center bg-white px-5 py-10 flex-grow w-full max-h-popup-height max-w-popup-width overflow-auto">
			{icon === 'success' ? (
				<div className="rounded-full w-9 h-9 flex items-center justify-center mb-5">
					<Handclap className="" />
				</div>
			) : null}
			{icon === 'bfc' ? (
				<div className="flex flex-col flex-nowrap items-center justify-center rounded-full w-16 h-16 bg-sui mb-7">
					<Logo className="bg-bfc rounded-full w-15 h-15" />
				</div>
			) : null}
			{headerCaption ? (
				<Text variant="caption" color="bfc-text1" weight="normal">
					{headerCaption}
				</Text>
			) : null}
			{title ? (
				<div className="text-center w-[280px]">
					<Heading variant="heading3" color="black" as="h1" weight="bold" leading="none">
						{title}
					</Heading>
				</div>
			) : null}
			{subtitle ? (
				<div className="text-center mb-3.75">
					<Text variant="caption" color="steel-darker" weight="bold">
						{subtitle}
					</Text>
				</div>
			) : null}
			{children}
		</div>
	);
}
