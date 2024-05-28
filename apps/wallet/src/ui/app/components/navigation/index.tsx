// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import cl from 'classnames';
import { memo } from 'react';
import { NavLink } from 'react-router-dom';

import { useAppSelector } from '_hooks';
import { getNavIsVisible } from '_redux/slices/app';

import st from './Navigation.module.scss';

function makeLinkCls({ isActive }: { isActive: boolean }) {
	return cl(st.link, { [st.active]: isActive });
}

export type NavigationProps = {
	className?: string;
};

function Navigation({ className }: NavigationProps) {
	const isVisible = useAppSelector(getNavIsVisible);
	return (
		<nav
			className={cl('h-20', st.container, className, {
				[st.hidden]: !isVisible,
			})}
		>
			<NavLink data-testid="nav-tokens" to="./tokens" className={makeLinkCls} title="Tokens">
				<svg
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M12 4C18.075 4 23 6.686 23 10V14C23 17.314 18.075 20 12 20C6.033 20 1.176 17.409 1.005 14.177L1 14V10C1 6.686 5.925 4 12 4ZM12 16C8.28 16 4.99 14.993 3 13.45V14C3 15.882 6.883 18 12 18C17.01 18 20.838 15.97 20.995 14.118L21 14L21.001 13.45C19.011 14.992 15.721 16 12 16ZM12 6C6.883 6 3 8.118 3 10C3 11.882 6.883 14 12 14C17.117 14 21 11.882 21 10C21 8.118 17.117 6 12 6Z"
						fill="currentColor"
					/>
				</svg>
				<span className={st.title}>Coins</span>
			</NavLink>
			<NavLink
				data-testid="nav-activity"
				to="./transactions"
				className={makeLinkCls}
				title="Transactions"
			>
				<svg
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M9 7.53894L15 21.5389L18.659 12.9999H23V10.9999H17.341L15 16.4609L9 2.46094L5.341 10.9999H1V12.9999H6.659L9 7.53894Z"
						fill="currentColor"
					/>
				</svg>
				<span className={st.title}>Activity</span>
			</NavLink>
=======
import { useAppSelector } from '_hooks';
import { getNavIsVisible } from '_redux/slices/app';
import { Activity32, Apps32, Nft132, Tokens32 } from '@mysten/icons';
import cl from 'clsx';
import { NavLink } from 'react-router-dom';

import { useActiveAccount } from '../../hooks/useActiveAccount';
import st from './Navigation.module.scss';

export function Navigation() {
	const isVisible = useAppSelector(getNavIsVisible);
	const activeAccount = useActiveAccount();
	const makeLinkCls = ({ isActive }: { isActive: boolean }) =>
		cl(st.link, { [st.active]: isActive, [st.disabled]: activeAccount?.isLocked });
	const makeLinkClsNoDisabled = ({ isActive }: { isActive: boolean }) =>
		cl(st.link, { [st.active]: isActive });
	return (
		<nav
			className={cl('border-b-0 rounded-tl-md rounded-tr-md shrink-0', st.container, {
				[st.hidden]: !isVisible,
			})}
		>
			<div id="sui-apps-filters" className="flex whitespace-nowrap w-full justify-center"></div>
			<div className={st.navMenu}>
				<NavLink
					data-testid="nav-tokens"
					to="./tokens"
					className={makeLinkClsNoDisabled}
					title="Home"
				>
					<Tokens32 className="w-8 h-8" />
					<span className={st.title}>Home</span>
				</NavLink>
				<NavLink
					to="./nfts"
					className={makeLinkCls}
					title="Assets"
					onClick={(e) => {
						if (activeAccount?.isLocked) {
							e.preventDefault();
						}
					}}
				>
					<Nft132 className="w-8 h-8" />
					<span className={st.title}>Assets</span>
				</NavLink>
				<NavLink
					to="./apps"
					className={makeLinkCls}
					title="Apps"
					onClick={(e) => {
						if (activeAccount?.isLocked) {
							e.preventDefault();
						}
					}}
				>
					<Apps32 className="w-8 h-8" />
					<span className={st.title}>Apps</span>
				</NavLink>
				<NavLink
					data-testid="nav-activity"
					to="./transactions"
					className={makeLinkCls}
					title="Transactions"
					onClick={(e) => {
						if (activeAccount?.isLocked) {
							e.preventDefault();
						}
					}}
				>
					<Activity32 className="w-8 h-8" />
					<span className={st.title}>Activity</span>
				</NavLink>
			</div>
>>>>>>> mainnet-v1.24.1
		</nav>
	);
}
