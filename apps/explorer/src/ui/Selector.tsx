// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { forwardRef } from 'react';

import { Label } from './utils/Label';

import type { ComponentProps } from 'react';

export interface SelectProps extends Omit<ComponentProps<'select'>, 'ref' | 'className'> {
	label: string;
	options: { label: string; value: string | number }[];
}

export const Selector = forwardRef<HTMLSelectElement, SelectProps>(
	({ label, options, ...selectProps }, ref) => (
		<Label label={label}>
			<select ref={ref} {...selectProps}>
				{options.map((i) => (
					<option key={i.value} value={i.value}>
						{i.label}
					</option>
				))}
			</select>
		</Label>
	),
);
