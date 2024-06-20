// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import ExplorerLink from '_src/ui/app/components/explorer-link';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { Text } from '_src/ui/app/shared/text';
import { formatAddress } from '@benfen/bfc.js/utils';
import { Disclosure } from '@headlessui/react';
import {
	getObjectChangeLabel,
	type ObjectChangesByOwner,
	type ObjectChangeSummary,
	type SuiObjectChangeTypes,
	type SuiObjectChangeWithDisplay,
} from '@mysten/core';
import { ChevronDown14, ChevronRight14 } from '@mysten/icons';
import cx from 'classnames';

import { ExpandableList } from '../../ExpandableList';
import { Card } from '../Card';
import { OwnerFooter } from '../OwnerFooter';
import { ObjectChangeDisplay } from './objectSummary/ObjectChangeDisplay';

function ChevronDown({ expanded }: { expanded: boolean }) {
	return expanded ? (
		<ChevronDown14 className="text-bfc-text2" />
	) : (
		<ChevronRight14 className="text-bfc-text2" />
	);
}

export function ObjectDetail({
	change,
	display,
}: {
	change: SuiObjectChangeWithDisplay;
	ownerKey: string;
	display?: boolean;
}) {
	if (change.type === 'transferred' || change.type === 'published') {
		return null;
	}

	const [packageId, moduleName, typeName] = change.objectType?.split('<')[0]?.split('::') || [];

	return (
		<Disclosure>
			{({ open }) => (
				<div className="px-2.5 pb-2.5 flex flex-col gap-2.5">
					<div className="grid grid-cols-2 overflow-auto cursor-pointer">
						<Disclosure.Button className="flex items-center cursor-pointer border-none bg-transparent ouline-none p-0 gap-1 text-bfc-text1 select-none">
							<Text variant="body" weight="normal">
								Object
							</Text>
							{open ? (
								<ChevronDown14 className="text-bfc-text2" />
							) : (
								<ChevronRight14 className="text-bfc-text2" />
							)}
						</Disclosure.Button>
						{change.objectId && (
							<div className="justify-self-end">
								<ExplorerLink
									type={ExplorerLinkType.object}
									objectID={change.objectId}
									className="no-underline"
								>
									<Text variant="body" weight="medium" color="bfc-text1" truncate>
										{formatAddress(change.objectId)}
									</Text>
								</ExplorerLink>
							</div>
						)}
					</div>
					<Disclosure.Panel>
						<div className="flex flex-col gap-2.5">
							<div className="grid grid-cols-2 overflow-auto relative">
								<Text variant="body" weight="normal" color="bfc-text2">
									Package
								</Text>
								<div className="flex justify-end">
									<ExplorerLink
										type={ExplorerLinkType.object}
										objectID={packageId}
										className="no-underline justify-self-end overflow-auto"
									>
										<Text variant="body" weight="medium" color="bfc-text1" truncate>
											{packageId}
										</Text>
									</ExplorerLink>
								</div>
							</div>
							<div className="grid grid-cols-2 overflow-auto">
								<Text variant="body" weight="normal" color="bfc-text2">
									Module
								</Text>
								<div className="flex justify-end">
									<ExplorerLink
										type={ExplorerLinkType.object}
										objectID={packageId}
										moduleName={moduleName}
										className="no-underline justify-self-end overflow-auto"
									>
										<Text variant="body" weight="medium" color="bfc-text1" truncate>
											{moduleName}
										</Text>
									</ExplorerLink>
								</div>
							</div>
							<div className="grid grid-cols-2 overflow-auto">
								<Text variant="body" weight="normal" color="bfc-text2">
									Type
								</Text>
								<div className="flex justify-end">
									<ExplorerLink
										type={ExplorerLinkType.object}
										objectID={packageId}
										moduleName={moduleName}
										className="no-underline justify-self-end overflow-auto"
									>
										<Text variant="body" weight="medium" color="bfc-text1" truncate>
											{typeName}
										</Text>
									</ExplorerLink>
								</div>
							</div>
						</div>
					</Disclosure.Panel>
				</div>
			)}
		</Disclosure>
	);
}

interface ObjectChangeEntryProps {
	type: SuiObjectChangeTypes;
	changes: ObjectChangesByOwner;
}

export function ObjectChangeEntry({ changes, type }: ObjectChangeEntryProps) {
	return (
		<>
			{Object.entries(changes).map(([owner, changes]) => {
				return (
					<Card
						footer={<OwnerFooter owner={owner} ownerType={changes.ownerType} />}
						key={`${type}-${owner}`}
						heading="Changes"
					>
						<Disclosure defaultOpen>
							{({ open }) => (
								<div className={cx('flex flex-col')}>
									<Disclosure.Button
										as="div"
										className="mt-2.5 h-[34px] py-2 px-2.5 flex items-center w-full gap-1.25 cursor-pointer"
									>
										<Text variant="body" weight="medium" color="bfc-text1">
											{getObjectChangeLabel(type)}
										</Text>
										<div className="h-px bg-bfc-border w-full" />
										<ChevronDown expanded={open} />
									</Disclosure.Button>
									<Disclosure.Panel as="div" className="gap-2.5 flex flex-col">
										<>
											{!!changes.changesWithDisplay.length && (
												<div className="flex flex-col gap-2.5 overflow-y-auto">
													<ExpandableList
														defaultItemsToShow={5}
														items={
															open
																? changes.changesWithDisplay.map((change) => (
																		<ObjectChangeDisplay change={change} />
																  ))
																: []
														}
													/>
												</div>
											)}

											<div className="flex w-full flex-col gap-2.5">
												<ExpandableList
													defaultItemsToShow={5}
													items={
														open
															? changes.changes.map((change) => (
																	<ObjectDetail ownerKey={owner} change={change} />
															  ))
															: []
													}
												/>
											</div>
										</>
									</Disclosure.Panel>
								</div>
							)}
						</Disclosure>
					</Card>
				);
			})}
		</>
	);
}

export function ObjectChanges({ changes }: { changes?: ObjectChangeSummary | null }) {
	if (!changes) return null;

	return (
		<>
			{Object.entries(changes).map(([type, changes]) => {
				return (
					<ObjectChangeEntry
						key={type}
						type={type as keyof ObjectChangeSummary}
						changes={changes}
					/>
				);
			})}
		</>
	);
}
