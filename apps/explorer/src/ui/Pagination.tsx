// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { PaginationFirst18, PaginationNext18, PaginationPrev18 } from '@mysten/icons';
import { type UseInfiniteQueryResult } from '@tanstack/react-query';
import { useState } from 'react';

export interface PaginationProps {
	hasPrev: boolean;
	hasNext: boolean;
	onFirst(): void;
	onPrev(): void;
	onNext(): void;
}

interface CursorPaginationProps extends PaginationProps {
	currentPage: number;
}

export interface PaginationResponse<Cursor = string> {
	nextCursor: Cursor | null;
	hasNextPage: boolean;
}

export function useCursorPagination<T>(query: UseInfiniteQueryResult<T>) {
	const [currentPage, setCurrentPage] = useState(0);

	return {
		...query,
		data: query.data?.pages[currentPage],
		pagination: {
			onFirst: () => setCurrentPage(0),
			onNext: () => {
				if (!query.data || query.isFetchingNextPage) {
					return;
				}

				// Make sure we are at the end before fetching another page
				if (currentPage >= query.data.pages.length - 1) {
					query.fetchNextPage();
				}

				setCurrentPage(currentPage + 1);
			},
			onPrev: () => {
				setCurrentPage(Math.max(currentPage - 1, 0));
			},
			hasNext:
				!query.isFetchingNextPage &&
				(currentPage < (query.data?.pages.length ?? 0) - 1 || !!query.hasNextPage),
			hasPrev: currentPage !== 0,
			currentPage,
		} satisfies CursorPaginationProps,
	};
}

/** @deprecated Prefer `useCursorPagination` + `useInfiniteQuery` for pagination. */
export function usePaginationStack<Cursor = string>() {
	const [stack, setStack] = useState<Cursor[]>([]);

	return {
		cursor: stack.at(-1),
		props({
			hasNextPage = false,
			nextCursor = null,
		}: Partial<PaginationResponse<Cursor>> = {}): PaginationProps {
			return {
				hasPrev: stack.length > 0,
				hasNext: hasNextPage,
				onFirst() {
					setStack([]);
				},
				onNext() {
					if (nextCursor && hasNextPage) {
						setStack((stack) => [...stack, nextCursor]);
					}
				},
				onPrev() {
					setStack((stack) => stack.slice(0, -1));
				},
			};
		},
	};
}

function PaginationButton({
	label,
	icon: Icon,
	disabled,
	onClick,
}: {
	label: string;
	icon: typeof PaginationFirst18;
	disabled: boolean;
	onClick(): void;
}) {
	return (
		<button
			className="rounded-md border border-bfc-border px-2 py-1 text-bfc-text1 shadow-xs disabled:text-bfc-text3"
			aria-label={label}
			type="button"
			disabled={disabled}
			onClick={onClick}
		>
			<Icon className="text-[24px]" />
		</button>
	);
}

export function Pagination({ hasNext, hasPrev, onFirst, onPrev, onNext }: PaginationProps) {
	return (
		<div className="flex gap-2">
			<PaginationButton
				label="Go to First"
				icon={PaginationFirst18}
				disabled={!hasPrev}
				onClick={onFirst}
			/>
			<PaginationButton
				label="Previous"
				icon={PaginationPrev18}
				disabled={!hasPrev}
				onClick={onPrev}
			/>
			<PaginationButton label="Next" icon={PaginationNext18} disabled={!hasNext} onClick={onNext} />
		</div>
	);
}
