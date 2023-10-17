// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Search16 } from '@mysten/icons';
import { Text, Combobox, ComboboxInput, ComboboxList } from '@mysten/ui';

export type SearchResult = {
	id: string;
	label: string;
	type: string;
};

export interface SearchProps {
	onChange: (value: string) => void;
	onSelectResult?: (result: SearchResult) => void;
	placeholder?: string;
	isLoading: boolean;
	options?: SearchResult[];
	queryValue: string;
}

export function Search({
	onChange,
	onSelectResult,
	placeholder,
	options = [],
	isLoading = false,
	queryValue,
}: SearchProps) {
	return (
		<Combobox value={queryValue} onValueChange={onChange}>
			<div className="relative flex items-center">
				<div className="absolute left-0 ml-1.5 block items-center text-2xl text-bfc-text3">
					<Search16 />
				</div>

				<ComboboxInput
					className="w-full rounded-lg border border-transparent bg-bfc-card pl-8 text-body leading-9 text-bfc-text1 outline-none placeholder:text-xs placeholder:text-bfc-text3"
					placeholder={placeholder}
				/>
			</div>

			<ComboboxList
				isLoading={isLoading}
				onSelect={({ item }) => {
					onSelectResult?.(item);
				}}
				options={options.map((item) => ({
					item,
					value: `${item.type}/${item.id}`,
					label: item.label,
					after: (
						<Text variant="caption/medium" color="steel">
							{item.type}
						</Text>
					),
				}))}
			/>
		</Combobox>
	);
}
