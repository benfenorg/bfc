// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type ReactNode } from 'react';

import PageTitle, { type PageTitleProps } from '_src/ui/app/shared/PageTitle';

export interface MenuLayoutProps extends PageTitleProps {
	children: ReactNode;
}

export function MenuLayout({ children, ...pageTitleProps }: MenuLayoutProps) {
	return (
		<>
			<div className="sticky top-0 bg-white">
				<PageTitle {...pageTitleProps} />
			</div>
			<div className="mt-5 flex flex-col justify-items-stretch flex-1">{children}</div>
		</>
	);
}
