// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Check12, ChevronDown16 } from '@mysten/icons';
import { Text } from '@mysten/ui';
import { useState, useRef, type ComponentProps, useCallback, useEffect } from 'react';

import { Label } from './utils/Label';

export interface SelectProps extends Omit<ComponentProps<'select'>, 'ref' | 'className'> {
	label: string;
}

export function Selector<T extends string | number>({
	label,
	options,
	value,
	onChange,
}: {
	label: string;
	options: { label: string; value: T }[];
	value?: T;
	onChange: (next: T | undefined) => void;
}) {
	const [selected, setSelected] = useState<T>();
	const [isOpen, setIsOpen] = useState(false);

	const container = useRef<HTMLDivElement>(null);

	const onSelect = useCallback(
		(value: T | undefined) => {
			setSelected(value);
			setIsOpen(false);
			onChange?.(value);
		},
		[onChange],
	);

	useEffect(() => {
		if (!options.find((i) => i.value === selected)) {
			onSelect(undefined);
		}
	}, [selected, options, onSelect]);

	useEffect(() => {
		setSelected((pre) => value ?? pre);
	}, [value]);

	useEffect(() => {
		if (isOpen) {
			const listener = (e: MouseEvent) => {
				if (!container.current?.contains(e.target as Node)) {
					setIsOpen(false);
					document.removeEventListener('click', listener);
				}
			};
			document.addEventListener('click', listener);
			return () => {
				document.removeEventListener('click', listener);
			};
		}
		return () => {};
	}, [isOpen]);

	return (
		<Label label={label}>
			<div className="relative" ref={container}>
				<div
					className="flex h-10 w-full flex-nowrap items-center gap-1 overflow-hidden bg-white p-2 text-bfc"
					onClick={() => setIsOpen((pre) => !pre)}
				>
					<span className="flex-grow">
						<Text variant="body/normal">
							{options?.find((i) => i.value === value ?? selected)?.label}
						</Text>
					</span>
					<ChevronDown16
						className="pointer-events-none h-4 w-4 text-bfc-text2 transition-all group-hover:text-bfc"
						aria-hidden="true"
					/>
				</div>
				{isOpen && (
					<div className="absolute left-0 top-10 z-10 max-h-60 w-full overflow-auto rounded-lg bg-white p-2 shadow">
						{options.map((option) => (
							<div
								key={option.value}
								className="flex flex-1 cursor-pointer flex-nowrap items-center gap-4 rounded-sm p-2 hover:bg-sui-light/40"
								onClick={() => {
									onSelect(option.value);
								}}
							>
								<span className="flex-1">
									<Text
										variant="body/normal"
										color={selected === option.value ? 'bfc-text1' : 'bfc-text2'}
										truncate
									>
										{option.label}
									</Text>
								</span>
								{selected === option.value ? (
									<Check12 className="h-4 w-4 text-steel-darker" aria-hidden="true" />
								) : null}
							</div>
						))}
					</div>
				)}
			</div>
		</Label>
	);
}
