// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/**
 * TODO: Generalize this file into shared components.
 */

import type { ReactNode } from 'react';

export interface DescriptionItemProps {
	title: string | ReactNode;
	children: ReactNode;
}

export function DescriptionItem({ title, children }: DescriptionItemProps) {
	return (
		<div className="flex items-center">
			<dt className="text-body font-medium text-bfc-text2 flex-1">{title}</dt>
			<dd className="ml-0 text-bfc-text1 text-body font-medium">{children}</dd>
		</div>
	);
}

export type DescriptionListProps = {
	children: ReactNode;
};

export function DescriptionList({ children }: DescriptionListProps) {
	return <dl className="flex flex-col gap-2.5 m-0">{children}</dl>;
}
