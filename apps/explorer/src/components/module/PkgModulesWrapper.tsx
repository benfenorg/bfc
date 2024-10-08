// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Search24 } from '@mysten/icons';
import { Heading, Combobox, ComboboxInput, ComboboxList } from '@mysten/ui';
import clsx from 'clsx';
import { useState, useCallback, useEffect } from 'react';
import { type Direction } from 'react-resizable-panels';

import ModuleView from './ModuleView';
import { ModuleFunctionsInteraction } from './module-functions-interaction';
import { useBreakpoint } from '~/hooks/useBreakpoint';
import { SplitPanes } from '~/ui/SplitPanes';
import { ListItem, VerticalList } from '~/ui/VerticalList';
import { useSearchParamsMerged } from '~/ui/utils/LinkWithQuery';

type ModuleType = [moduleName: string, code: string];

interface Props {
	id?: string;
	modules: ModuleType[];
	splitPanelOrientation: Direction;
}

interface ModuleViewWrapperProps {
	id?: string;
	selectedModuleName: string;
	modules: ModuleType[];
}

function ModuleViewWrapper({ id, selectedModuleName, modules }: ModuleViewWrapperProps) {
	const selectedModuleData = modules.find(([name]) => name === selectedModuleName);

	if (!selectedModuleData) {
		return null;
	}

	const [name, code] = selectedModuleData;

	return <ModuleView id={id} name={name} code={code} />;
}

function PkgModuleViewWrapper({ id, modules, splitPanelOrientation }: Props) {
	const isMediumOrAbove = useBreakpoint('md');

	const modulenames = modules.map(([name]) => name);
	const [searchParams, setSearchParams] = useSearchParamsMerged();
	const [query, setQuery] = useState('');

	// Extract module in URL or default to first module in list
	const selectedModule =
		searchParams.get('module') && modulenames.includes(searchParams.get('module')!)
			? searchParams.get('module')!
			: modulenames[0];

	// If module in URL exists but is not in module list, then delete module from URL
	useEffect(() => {
		if (searchParams.has('module') && !modulenames.includes(searchParams.get('module')!)) {
			setSearchParams({}, { replace: true });
		}
	}, [searchParams, setSearchParams, modulenames]);

	const filteredModules =
		query === ''
			? modulenames
			: modules
					.filter(([name]) => name.toLowerCase().includes(query.toLowerCase()))
					.map(([name]) => name);

	const submitSearch = useCallback(() => {
		if (filteredModules.length === 1) {
			setSearchParams({
				module: filteredModules[0],
			});
		}
	}, [filteredModules, setSearchParams]);

	const onChangeModule = (newModule: string) => {
		setSearchParams(
			{
				module: newModule,
			},
			{
				preventScrollReset: true,
			},
		);
	};

	const bytecodeContent = [
		{
			panel: (
				<div
					key="bytecode"
					className="h-full grow overflow-auto rounded-md border border-bfc-border px-4 py-5"
				>
					<Heading variant="heading4/semibold" color="steel-darker">
						Bytecode
					</Heading>
					<div
						className={clsx(
							'mt-5 overflow-auto rounded-md bg-bfc-card p-4',
							(splitPanelOrientation === 'horizontal' || !isMediumOrAbove) && 'h-verticalListLong',
						)}
					>
						<ModuleViewWrapper id={id} modules={modules} selectedModuleName={selectedModule} />
					</div>
				</div>
			),
			defaultSize: 40,
		},
		{
			panel: (
				<div
					key="execute"
					className="pb-15 h-full grow overflow-auto rounded-md border border-bfc-border px-4 pt-5"
				>
					<Heading variant="heading4/semibold" color="steel-darker">
						Execute
					</Heading>
					<div
						className={clsx(
							'mt-5 overflow-auto',
							(splitPanelOrientation === 'horizontal' || !isMediumOrAbove) && 'h-verticalListLong',
						)}
					>
						{id && selectedModule ? (
							<ModuleFunctionsInteraction
								// force recreating everything when we change modules
								key={`${id}-${selectedModule}`}
								packageId={id}
								moduleName={selectedModule}
							/>
						) : null}
					</div>
				</div>
			),
			defaultSize: 60,
		},
	];

	return (
		<div className="mt-5 flex flex-col gap-5 md:flex-row md:flex-nowrap">
			<div className="w-full rounded-md border border-bfc-border p-2 md:w-1/5">
				<Combobox value={query} onValueChange={setQuery}>
					<div className="flex w-full justify-between overflow-hidden rounded-md border border-bfc-border bg-bfc-card pl-3 placeholder-bfc-text3">
						<ComboboxInput
							placeholder="Search"
							className="w-full border-none bg-bfc-card text-body leading-9 text-bfc-text1 outline-none placeholder:text-xs placeholder:text-bfc-text3"
						/>
						<button onClick={submitSearch} className="border-none bg-inherit pr-2" type="submit">
							<Search24 className="h-4.5 w-4.5 cursor-pointer fill-steel align-middle text-gray-60" />
						</button>
					</div>

					<ComboboxList
						showResultsCount
						options={filteredModules.map((item) => ({ item, value: item, label: item }))}
						onSelect={({ item }) => {
							onChangeModule(item);
						}}
					/>
				</Combobox>
				<div className="h-verticalListShort overflow-auto pt-3 md:h-verticalListLong">
					<VerticalList>
						{modulenames.map((name) => (
							<div key={name} className="mx-0.5 mt-0.5 md:min-w-fit">
								<ListItem
									active={selectedModule === name}
									type="borderLine"
									onClick={() => onChangeModule(name)}
								>
									{name}
								</ListItem>
							</div>
						))}
					</VerticalList>
				</div>
			</div>
			{isMediumOrAbove ? (
				<div className="w-4/5">
					<SplitPanes direction={splitPanelOrientation} splitPanels={bytecodeContent} />
				</div>
			) : (
				bytecodeContent.map((panel, index) => <div key={index}>{panel.panel}</div>)
			)}
		</div>
	);
}
export default PkgModuleViewWrapper;
