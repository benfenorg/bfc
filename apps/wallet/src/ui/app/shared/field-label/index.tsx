// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import type { ReactNode } from 'react';

export type FieldLabelProps = {
	txt: string;
	children: ReactNode | ReactNode[];
};

export default function FieldLabel({ txt, children }: FieldLabelProps) {
	return (
		<label className="flex flex-col flex-nowrap gap-1.25 mt-5 first:mt-0">
			<Text variant="bodySmall" color="bfc-text1" weight="normal">
				{txt}
			</Text>

			{children}
		</label>
	);
}
