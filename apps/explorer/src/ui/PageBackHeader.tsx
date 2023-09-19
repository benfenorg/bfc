// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ArrowLeft12 } from '@mysten/icons';
import { Heading } from '@mysten/ui';
import { useNavigate } from 'react-router-dom';

export type PageHeaderType = 'Transaction' | 'Checkpoint' | 'Address' | 'Object' | 'Package';

export interface PageHeaderProps {
	title: string;
	subtitle?: string | null;
	type?: PageHeaderType;
	status?: 'success' | 'failure';
}

export function PageBackHeader({ title }: PageHeaderProps) {
	const navigate = useNavigate();
	const pageBack = () => {
		navigate(-1);
	};

	return (
		<div data-testid="pagebackheader">
			<div className="mb-5 flex cursor-pointer items-center gap-2" onClick={pageBack}>
				<ArrowLeft12 width={20} height={20} className="text-obc-text1" />
				<Heading variant="heading4/semibold" color="obc-text1">
					{title}
				</Heading>
			</div>
		</div>
	);
}
