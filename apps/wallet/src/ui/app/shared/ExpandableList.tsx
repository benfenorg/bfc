// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ChevronDown14 } from '@mysten/icons';
import clsx from 'classnames';
import { useMemo, useState, type ReactNode } from 'react';

import { Link } from './Link';
import { Text } from './text';

interface ExpandableListProps {
	items: ReactNode[];
	defaultItemsToShow: number;
}

export function ExpandableList({ items, defaultItemsToShow }: ExpandableListProps) {
	const [showAll, setShowAll] = useState(false);

	const itemsDisplayed = useMemo(
		() => (showAll ? items : items?.slice(0, defaultItemsToShow)),
		[showAll, items, defaultItemsToShow],
	);

	const handleShowAllClick = () => setShowAll((prevShowAll: boolean) => !prevShowAll);

	return (
		<>
			{itemsDisplayed.map((item, index) => (
				<div key={index}>{item}</div>
			))}
			{items.length > defaultItemsToShow && (
				<div className="flex cursor-pointer items-center w-full">
					<Link
						onClick={handleShowAllClick}
						after={
							<ChevronDown14
								height={14}
								width={14}
								className={clsx('text-bfc-text2', {
									'rotate-180': showAll,
								})}
							/>
						}
					>
						<Text variant="body" color="bfc-text2" weight="normal">
							{showAll ? 'Show Less' : 'Show All'}
						</Text>
					</Link>
				</div>
			)}
		</>
	);
}
