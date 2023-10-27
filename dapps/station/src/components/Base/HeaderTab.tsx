// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useNavigate } from 'react-router-dom';
import { useEffect, useState } from 'react';
import clsx from 'clsx';
import { useAppSelector } from "~/state/hooks";

export function HeaderTab() {
	const [tab, setTab] = useState('/');

	const statePathTab = useAppSelector((state) => state.user.pathTab);
	const navigate = useNavigate();

	console.log('statePathTabstatePathTab',statePathTab)

	useEffect(()=>{
		setTab(statePathTab)
	},[statePathTab])

	const pathTab = [
		{ name: '稳定币', path: '/' },
		{ name: '兑换', path: '/swap' },
		{ name: '流动池', path: '/pool' },
	];

	const changePage = (path:string) => {
		setTab(path);
		navigate(path)
	}
	return (
		<div className="w-full flex justify-between items-center border-b border-bf-hover">
			<div className="flex gap-5">
				{pathTab.map((item) => (
					<div
						key={item.path}
						className={clsx(
							'py-4 text-sm text-bf-card cursor-pointer',
							tab === item.path && 'font-semibold text-white border-b border-white',
						)}
						onClick={() => {
							changePage(item.path)
						}}
					>
						{item.name}
					</div>
				))}
			</div>
			<div>啊啥的都是</div>
		</div>
	);
}
