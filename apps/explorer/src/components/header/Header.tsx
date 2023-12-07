// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Disclosure } from '@headlessui/react';
import {
	Close,
	Search24Dark,
	Search24Light,
	Options24Dark,
	Options24Light,
	ArrowDown20,
} from '@mysten/icons';
import clsx from 'clsx';
import { useEffect, useMemo, useState } from 'react';
import { useLocation } from 'react-router-dom';

import { HeaderLogo } from './HeaderLogo';
import NetworkSelect from '../network/Network';
import Search from '../search/Search';
import { Link } from '~/ui/Link';
import { PageSelect } from '~/ui/header/PageSelect';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';

const SECONDARY_TYPE = {
	PAGE_SELECT: 'page_select',
	SEARCH: 'search',
};

function Header() {
	const { pathname } = useLocation();
	const [isBlackHeader, setIsBlackHeader] = useState(true);
	const [secondaryType, setSecondaryType] = useState<string>();

	useEffect(() => {
		const callback = () => {
			const homeContent: any = document.getElementById('home-content');
			if (homeContent && homeContent?.offsetTop - window.scrollY - 68 > 0) {
				setIsBlackHeader(true);
			} else {
				setIsBlackHeader(false);
			}
		};
		callback();
		document.addEventListener('scroll', callback, { passive: true });
		return () => {
			document.removeEventListener('scroll', callback);
		};
	}, []);

	useEffect(() => {
		const callback = () => {
			if (window.innerWidth > 768) {
				setSecondaryType(undefined);
			}
		};

		callback();
		window.addEventListener('resize', callback, { passive: true });
		return () => {
			window.removeEventListener('resize', callback);
		};
	}, []);

	const showBlack = useMemo(() => {
		if ((pathname === '/' || pathname === '/packages' || pathname === '/dao') && isBlackHeader) {
			return true;
		}
		return false;
	}, [isBlackHeader, pathname]);

	if (secondaryType) {
		return (
			<div className="fixed bottom-0 left-0 right-0 top-0 flex flex-col items-stretch bg-white">
				<header className="sticky top-0 z-20 flex h-18 items-center justify-center gap-2.5 overflow-visible border-b border-[#E1E1E9] bg-white p-5 text-bfc-text2 backdrop-blur-xl transition-shadow">
					{secondaryType === SECONDARY_TYPE.PAGE_SELECT && (
						<div className="grow">
							<HeaderLogo isDarker={false} />
						</div>
					)}
					{secondaryType === SECONDARY_TYPE.SEARCH && (
						<div className="w-full">
							<Search />
						</div>
					)}
					<div
						className="flex h-8 w-8 cursor-pointer items-center justify-center rounded-md active:bg-bfc-hover"
						onClick={() => setSecondaryType(undefined)}
					>
						<Close />
					</div>
				</header>
				{secondaryType === SECONDARY_TYPE.PAGE_SELECT && (
					<div className="flex flex-col items-stretch px-5">
						<div className="py-[15px]">
							<NetworkSelect isDarker={false} />
						</div>
						<Link variant="content" to="/" onClick={() => setSecondaryType(undefined)}>
							<div
								className={clsx(
									'cursor-pointer rounded-md py-[15px] text-bodyLarge/[20px] font-semibold hover:bg-bfc-hover',
									pathname === '/' ? 'text-bfc' : undefined,
								)}
							>
								Home
							</div>
						</Link>
						<div className="h-px w-full bg-bfc-border" />
						<Disclosure>
							{({ open }) => (
								<div className="flex w-full flex-col items-stretch">
									<Disclosure.Button>
										<div
											className={clsx(
												'flex items-center py-[15px]',
												open ? '' : 'border-b border-bfc-border',
											)}
										>
											<span className="grow text-left text-bodyLarge/[20px] font-semibold">
												Blockchain
											</span>
											<ArrowDown20 className={clsx(open ? 'rotate-180' : '')} />
										</div>
									</Disclosure.Button>
									<Disclosure.Panel>
										<div className={clsx('flex flex-col items-stretch')}>
											{[
												{ id: '/recent?tab=checkpoints', label: 'Checkpoints' },
												{ id: '/recent', label: 'Transaction Blocks' },
												{ id: '/recent?tab=epochs', label: 'Epochs' },
												{ id: '/validators', label: 'Validators' },
												{ id: '/packages', label: 'Packages' },
											].map((item) => (
												<Link
													key={item.id}
													to={item.id}
													onClick={() => setSecondaryType(undefined)}
												>
													<div className="border-l border-bfc-border p-[15px] text-bodyLarge/[20px] font-semibold text-bfc">
														{item.label}
													</div>
												</Link>
											))}
										</div>
									</Disclosure.Panel>
								</div>
							)}
						</Disclosure>
						<Link
							variant="content"
							to="/dao"
							underline="underline"
							onClick={() => setSecondaryType(undefined)}
						>
							<div
								className={clsx(
									'cursor-pointer rounded-md py-[15px] text-bodyLarge/[20px] font-semibold hover:bg-bfc-hover',
									pathname === '/dao' ? 'text-bfc' : undefined,
								)}
							>
								Dao
							</div>
						</Link>
						<div className="h-px w-full bg-bfc-border" />
					</div>
				)}
			</div>
		);
	}

	return (
		<header
			className={clsx(
				'sticky top-0 z-20 flex h-18 justify-center overflow-visible backdrop-blur-xl transition-shadow ',
				showBlack
					? 'border-b-0 bg-bfc text-white/[0.72]'
					: 'border-b border-[#E1E1E9] bg-white text-bfc-text2',
			)}
		>
			<div className="flex h-full max-w-[1440px] flex-1 items-center justify-between gap-5 px-5 md:px-10">
				<LinkWithQuery data-testid="nav-logo-button" to="/">
					<HeaderLogo isDarker={showBlack} />
				</LinkWithQuery>
				<div className="hidden items-center gap-2.5 text-body font-medium md:flex">
					<Link variant="content" to="/">
						<div
							className={clsx(
								'cursor-pointer rounded-md p-[7px] hover:bg-bfc-hover max-md:hidden',
								pathname === '/' ? (showBlack ? 'text-white' : 'text-bfc') : undefined,
							)}
						>
							Home
						</div>
					</Link>
					<PageSelect
						isDarker={showBlack}
						highlight={['/recent', '/validators', '/packages'].includes(pathname)}
					/>
					<Link variant="content" to="/dao">
						<div
							className={clsx(
								'cursor-pointer rounded-md p-[7px] hover:bg-bfc-hover max-md:hidden',
								pathname === '/dao' ? (showBlack ? 'text-white' : 'text-bfc') : undefined,
							)}
						>
							Dao
						</div>
					</Link>
					<div className="mx-[25px] w-full flex-1 sm:w-[380px]">
						<Search />
					</div>
					<NetworkSelect isDarker={showBlack} />
				</div>
				<div className="flex items-center gap-2.5 text-body font-medium md:hidden">
					<div
						className={clsx(
							'flex h-8 w-8 cursor-pointer items-center justify-center rounded-md',
							showBlack ? 'active:bg-bfc-hover' : 'active:bg-[#A3A8B529]',
						)}
						onClick={() => setSecondaryType(SECONDARY_TYPE.SEARCH)}
					>
						{showBlack ? <Search24Dark /> : <Search24Light />}
					</div>
					<div
						className={clsx(
							'flex h-8 w-8 cursor-pointer items-center justify-center rounded-md',
							showBlack ? 'active:bg-bfc-hover' : 'active:bg-[#A3A8B529]',
						)}
						onClick={() => setSecondaryType(SECONDARY_TYPE.PAGE_SELECT)}
					>
						{showBlack ? <Options24Dark /> : <Options24Light />}
					</div>
				</div>
			</div>
		</header>
	);
}

export default Header;
