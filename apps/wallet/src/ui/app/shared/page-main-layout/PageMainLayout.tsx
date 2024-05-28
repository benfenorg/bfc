// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
<<<<<<< HEAD:apps/wallet/src/ui/app/shared/page-main-layout/index.tsx
import cl from 'classnames';
import { createContext, type ReactNode, useState } from 'react';
=======

import { ErrorBoundary } from '_components/error-boundary';
import { MenuContent } from '_components/menu';
import { Navigation } from '_components/navigation';
import cn from 'clsx';
import { createContext, useState, type ReactNode } from 'react';
>>>>>>> mainnet-v1.24.1:apps/wallet/src/ui/app/shared/page-main-layout/PageMainLayout.tsx

import { useAppSelector } from '../../hooks';
import { AppType } from '../../redux/slices/app/AppType';
import DappStatus from '../dapp-status';
import { Header } from '../header/Header';
import { Toaster } from '../toaster';
<<<<<<< HEAD:apps/wallet/src/ui/app/shared/page-main-layout/index.tsx
import { ErrorBoundary } from '_components/error-boundary';
import { MenuButton, MenuContent } from '_components/menu';
import Navigation from '_components/navigation';

import st from './PageMainLayout.module.scss';
=======
>>>>>>> mainnet-v1.24.1:apps/wallet/src/ui/app/shared/page-main-layout/PageMainLayout.tsx

export const PageMainLayoutContext = createContext<HTMLDivElement | null>(null);

export type PageMainLayoutProps = {
	children: ReactNode | ReactNode[];
	bottomNavEnabled?: boolean;
	topNavMenuEnabled?: boolean;
	dappStatusEnabled?: boolean;
};

export function PageMainLayout({
	children,
	bottomNavEnabled = false,
	topNavMenuEnabled = false,
	dappStatusEnabled = false,
}: PageMainLayoutProps) {
	const networkName = useAppSelector(({ app: { apiEnv } }) => apiEnv);
	const appType = useAppSelector((state) => state.app.appType);
	const isFullScreen = appType === AppType.fullscreen;
	const [titlePortalContainer, setTitlePortalContainer] = useState<HTMLDivElement | null>(null);
	return (
		<div
			className={cn(
				'flex flex-col flex-nowrap items-stretch justify-center flex-1 w-full max-h-full bg-gradients-graph-cards overflow-hidden',
				isFullScreen ? 'rounded-xl' : '',
			)}
		>
			<Header
				networkName={networkName}
				middleContent={dappStatusEnabled ? <DappStatus /> : <div ref={setTitlePortalContainer} />}
				rightContent={topNavMenuEnabled ? <MenuButton /> : undefined}
			/>
			<div className="relative flex flex-col flex-nowrap flex-grow overflow-hidden rounded-t-xl shadow-wallet-content">
				<div className="flex flex-col flex-nowrap bg-white flex-grow overflow-y-auto overflow-x-hidden rounded-t-xl">
					<main
						className={cn('flex flex-col flex-grow w-full', {
							'p-5': bottomNavEnabled,
						})}
					>
						<PageMainLayoutContext.Provider value={titlePortalContainer}>
							<ErrorBoundary>{children}</ErrorBoundary>
						</PageMainLayoutContext.Provider>
					</main>
					{bottomNavEnabled ? <Navigation /> : null}
					<Toaster bottomNavEnabled={bottomNavEnabled} />
				</div>
				{topNavMenuEnabled ? <MenuContent /> : null}
			</div>
		</div>
	);
}
