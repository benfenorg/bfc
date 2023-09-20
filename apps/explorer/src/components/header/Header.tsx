// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import clsx from 'clsx';
import { useEffect, useMemo, useState } from 'react';
import { useLocation } from 'react-router-dom';

import { HeaderLogo } from './HeaderLogo';
import NetworkSelect from '../network/Network';
import Search from '../search/Search';
import { Link } from '~/ui/Link';
import { PageSelect } from '~/ui/header/PageSelect';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';

function Header() {
	const [isBlackHeader, setIsBlackHeader] = useState(false);
	const { pathname } = useLocation();
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

	const showBlack = useMemo(() => {
		if ((pathname === '/' || pathname === '/packages' || pathname === '/dao') && isBlackHeader) {
			return true;
		}
		return false;
	}, [isBlackHeader, pathname]);

	return (
		<header
			className={clsx(
				'sticky top-0 z-20 flex h-header justify-center overflow-visible backdrop-blur-xl transition-shadow ',
				showBlack ? 'border-b-0 bg-obc text-white/75' : 'border-b border-[#E1E1E9] bg-white',
			)}
		>
			<div className="flex h-full max-w-[1440px] flex-1 items-center justify-between gap-5 px-5 2xl:p-0">
				<LinkWithQuery data-testid="nav-logo-button" to="/">
					<HeaderLogo isDarker={showBlack} />
				</LinkWithQuery>
				<div className="flex items-center gap-5 px-5 text-body max-md:p-0">
					<Link variant="content" to="/">
						<div className="cursor-pointer max-md:hidden">Home</div>
					</Link>
					<PageSelect isDarker={showBlack} />
					<Link variant="content" to="/dao">
						<div className="cursor-pointer max-md:hidden">Dao</div>
					</Link>
					<div className="w-full flex-1 sm:w-[380px]">
						<Search />
					</div>
					<NetworkSelect isDarker={showBlack} />
				</div>
			</div>
		</header>
	);
}

export default Header;
