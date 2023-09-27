// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Add16, Download16 } from '@mysten/icons';
import { Link } from 'react-router-dom';

import { ampli } from '_src/shared/analytics/ampli';

const selections = [
	{
		title: 'Yes, let’s create one!',
		desc: 'This creates a new wallet and a 12-word recovery phrase.',
		url: '../create',
		action: 'Create a New Wallet',
		onClick: () => ampli.clickedCreateNewWallet(),
		icon: <Add16 className="font-semibold" />,
	},
	{
		title: 'No, I already have one',
		desc: 'Import your existing wallet by entering the 12-word recovery phrase.',
		url: '../import',
		action: 'Import an Existing Wallet',
		onClick: () => ampli.clickedImportExistingWallet(),
		icon: <Download16 className="font-semibold" />,
	},
];

const SelectPage = () => {
	return (
		<>
			<div className="text-xl/[26px] text-white font-bold">
				New to <span className="text-obc-text3">XChain</span> Wallet?
			</div>
			<div className="flex flex-col flex-nowrap gap-5 mt-10">
				{selections.map((aSelection) => (
					<div
						className={
							'bg-obc-card flex flex-col flex-nowrap items-center text-center rounded-xl py-10 p-5 max-w-popup-width border border-obc-border'
						}
						key={aSelection.url}
					>
						<div className="text-base/[22px] font-semibold text-obc-text1">{aSelection.title}</div>
						<div className="grow mt-2.5 text-xs/[18px] font-normal text-obc-text2 text-center">
							{aSelection.desc}
						</div>

						<Link
							to={aSelection.url}
							onClick={aSelection.onClick}
							className={
								'mt-[25px] flex flex-nowrap items-center justify-center w-full h-10 gap-2.5 rounded-lg bg-obc no-underline font-medium text-white text-xs'
							}
						>
							{aSelection.icon}
							{aSelection.action}
						</Link>
					</div>
				))}
			</div>
		</>
	);
};

export default SelectPage;
