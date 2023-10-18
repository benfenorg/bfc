// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactNode, useState } from 'react';
import { ConnectModal, useWalletKit } from '@mysten/wallet-kit';
import { Button } from './Button';
import { Menu } from '@headlessui/react';
import { formatAddress } from '@mysten/sui.js/utils';
import { styled } from '@stitches/react';
import { ReactComponent as CheckIcon } from '../../assets/check_icon.svg';
import { ReactComponent as ChevronIcon } from '../../assets/chevron_icon.svg';

interface ConnectButtonProps {
	connectText?: ReactNode;
	connectedText?: ReactNode;
}

const MenuItems = styled(Menu.Items, {
	position: 'absolute',
	right: 0,
	marginTop: '$1',
	width: 180,
	maxHeight: 200,
	overflow: 'scroll',
	borderRadius: '$buttonLg',
	backgroundColor: '$background',
	color: '$textDark',
	boxShadow: '$button',
	zIndex: 10,
	padding: '$2',
	display: 'flex',
	flexDirection: 'column',
	gap: '$2',
});

const Account = styled('button', {
	border: 0,
	display: 'flex',
	justifyContent: 'space-between',
	alignItems: 'center',
	backgroundColor: 'white',
	fontFamily: '$mono',
	padding: '$2',
	color: '#758F9E',
	cursor: 'pointer',
	textAlign: 'left',
	fontSize: 14,
	borderRadius: 3,

	'&:hover': {
		color: '#0284AD',
		backgroundColor: '#E1F3FF80',
	},

	variants: {
		active: {
			true: {
				color: '#007195',
			},
		},
	},
});

export function ConnectButton({
	connectText = 'Connect Wallet',
	connectedText,
}: ConnectButtonProps) {
	const [connectModalOpen, setConnectModalOpen] = useState(false);
	const { currentAccount, accounts, selectAccount, disconnect } = useWalletKit();

	return (
		<>
			{currentAccount ? (
				<Menu as="div" style={{ position: 'relative', display: 'inline-block' }}>
					<Menu.Button
						className="inline-flex w-full justify-center bg-white rounded-lg px-3 py-2 font-semibold text-bf-text1 text-xs"
					>
						{connectedText ?? formatAddress(currentAccount.address)}
						<ChevronIcon />
					</Menu.Button>

					<MenuItems>
						{accounts.map((account: any) => (
							<Menu.Item key={account.address}>
								<Account
									active={account.address === currentAccount.address}
									onClick={() => selectAccount(account)}
								>
									{formatAddress(account.address)}

									{account.address === currentAccount.address && <CheckIcon />}
								</Account>
							</Menu.Item>
						))}

						<div
							style={{
								marginTop: 4,
								marginBottom: 4,
								height: 1,
								background: '#F3F6F8',
								flexShrink: 0,
							}}
						/>

						<Menu.Item>
							<Account css={{ fontFamily: '$sans' }} onClick={() => disconnect()}>
								Disconnect
							</Account>
						</Menu.Item>
					</MenuItems>
				</Menu>
			) : (
				// <Button
				// 	color="primary"
				// 	size="lg"
				// 	onClick={() => setConnectModalOpen(true)}
				// 	type="button"
				// 	{...props}
				// >
				// 	{connectText}
				// </Button>

				<button className="bg-white text-bf-text1 px-3 py-2 rounded-lg font-semibold" 
				onClick={() => setConnectModalOpen(true)}>
					{connectText}
				</button>
			)}

			{!currentAccount && (
				<ConnectModal open={connectModalOpen} onClose={() => setConnectModalOpen(false)} />
			)}
		</>
	);
}
