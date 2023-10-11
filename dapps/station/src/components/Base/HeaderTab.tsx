// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useNavigate } from 'react-router-dom';
import { useState } from 'react';
import clsx from 'clsx';

export function HeaderTab() {
	const navigate = useNavigate();

	const [tab, setTab] = useState('1');

	const pathTab = [
		{ name: '稳定币', path: '1' },
		{ name: '兑换', path: '2' },
		{ name: '流动池', path: '3' },
	];

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
							setTab(item.path);
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
