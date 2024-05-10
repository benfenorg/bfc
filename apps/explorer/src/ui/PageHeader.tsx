// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Heading } from '@mysten/ui';

import { Badge } from './Badge';
import { CopyToClipboard } from '~/ui/CopyToClipboard';

export type PageHeaderType =
	| 'Transaction'
	| 'Checkpoint'
	| 'Address'
	| 'Object'
	| 'Package'
	| 'Dao';

export interface PageHeaderProps {
	title?: string;
	subtitle?: string | null;
	type: PageHeaderType;
	status?: 'success' | 'failure';
	statusMessage?: string;
	loading?: boolean;
}

const TYPE_TO_COPY: Partial<Record<PageHeaderType, string>> = {
	Transaction: 'Transaction Block',
	Dao: 'DAO Proposals Details',
};

const STATUS_TO_TEXT = {
	success: 'Success',
	failure: 'Failure',
};

export function PageHeader({
	title,
	subtitle,
	type,
	status,
	statusMessage,
	loading,
}: PageHeaderProps) {
	return (
		<div data-testid="pageheader">
			<div className="mb-5 flex items-center gap-2">
				<Heading variant="heading4/semibold" color="bfc-text1">
					{type in TYPE_TO_COPY ? TYPE_TO_COPY[type] : type}
				</Heading>
			</div>
			{title && (
				<div className="flex flex-col gap-2 rounded-md border-l-4 border-bfc-border bg-bfc-card p-5 lg:flex-row">
					<div className="flex min-w-0 items-center gap-2">
						<div className="min-w-0 break-words">
							<Heading as="h2" variant="heading3/semibold" color="bfc-text1" mono>
								{title}
							</Heading>
						</div>
						<CopyToClipboard size="lg" color="steel" copyText={title} />
					</div>

					{status && (
						<div className="flex items-center gap-2.5">
							<Badge variant={status}>{STATUS_TO_TEXT[status]}</Badge>
							{statusMessage && <Badge variant={status}>{statusMessage}</Badge>}
						</div>
					)}
				</div>
			)}
			{subtitle && (
				<div className="mt-2 break-words">
					<Heading variant="heading4/semibold" color="gray-75">
						{subtitle}
					</Heading>
				</div>
			)}
		</div>
	);
}
