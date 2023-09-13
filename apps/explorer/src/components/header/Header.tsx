// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Sui, SuiLogoTxt } from '@mysten/icons';
import clsx from 'clsx';
import { useEffect, useMemo, useState } from 'react';
import { Link } from '~/ui/Link';
import NetworkSelect from '../network/Network';
import {PageSelect} from '~/ui/header/PageSelect';
import Search from '../search/Search';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';
import { useLocation } from 'react-router-dom';
import { HeaderLogo } from './HeaderLogo';

function Header() {
	const [isBlackHeader, setIsBlackHeader] = useState(false);
	const {pathname} = useLocation();
	useEffect(() => {
		const callback = () => {
			const homeContent:any = document.getElementById('home-content')
			if(homeContent && homeContent?.offsetTop - window.scrollY - 68 > 0){
				setIsBlackHeader(true)
			} else {
				setIsBlackHeader(false)
			}
		};
		callback();
		document.addEventListener('scroll', callback, { passive: true });
		return () => {
			document.removeEventListener('scroll', callback);
		};
	}, []);

	const showBlack = useMemo(()=>{
		if((pathname === '/' || pathname === '/packages' || pathname === '/dao') && isBlackHeader){
			return true
		}
		return false
	},[isBlackHeader,pathname])

	return (
		<header
			className={clsx(
				'sticky top-0 z-20 flex h-header justify-center overflow-visible backdrop-blur-xl transition-shadow ',
				showBlack ? 'bg-obc border-b-0 text-white/75' : 'bg-white border-b border-[#E1E1E9]'
			)}
		>
			<div className="flex justify-between h-full max-w-[1440px] flex-1 items-center gap-5 px-5 2xl:p-0">
				<LinkWithQuery
					data-testid="nav-logo-button"
					to="/"
				>
					<HeaderLogo isDarker={showBlack}/>
				</LinkWithQuery>
				<div className="flex text-body items-center gap-5 px-5 max-md:p-0">
					<Link variant='content' to={'/'}>
						<div className='cursor-pointer max-md:hidden'>Home</div>
					</Link>
					<PageSelect isDarker={showBlack}/>
					<Link variant='content' to={'/dao'}>
						<div className='cursor-pointer max-md:hidden'>Dao</div>
					</Link>
					<div className="w-full flex-1 sm:w-[380px]">
						<Search />
					</div>
					<NetworkSelect isDarker={showBlack}/>
				</div>
				
			</div>
		</header>
	);
}

export default Header;
