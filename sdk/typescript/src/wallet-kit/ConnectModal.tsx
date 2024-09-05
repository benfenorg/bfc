// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Dialog } from '@headlessui/react';
import { useEffect, useState } from 'react';
import { styled } from './stitches.js';
import { Button, Panel } from './utils/ui.js';
import { BackIcon } from './utils/icons.js';
import type { Props as WhatIsAWalletProps } from './WhatIsAWallet.js';
import { WhatIsAWallet } from './WhatIsAWallet.js';
import { Body, CloseButton, Content, Overlay, Title } from './utils/Dialog.js';
import { SELECTED_GETTING_STARTED, WalletList } from './WalletList.js';
import type { Props as GettingStartedProps } from './GettingStarted.js';
import { GettingStarted } from './GettingStarted.js';
import { useWalletKit } from './WalletKitContext.js';

export interface ConnectModalProps {
	open: boolean;
	onClose(): void;
	onError(err: Error): void;
	text?: {
		opening: string;
		connectionFailed: string;
		confirmConnection: string;
		retryConnection: string;
		getStarted: string;
		installWallet: string;
		whatIsAWallet?: WhatIsAWalletProps & {
			title: string;
		};
		walletList?: {
			connectWallet: string;
		};
		gettingStarted?: GettingStartedProps;
	};
}

const BackButton = styled('button', {
	position: 'absolute',
	cursor: 'pointer',
	top: '$4',
	left: '$4',
	display: 'flex',
	border: 'none',
	alignItems: 'center',
	justifyContent: 'center',
	color: '$icon',
	backgroundColor: 'transparent',

	'@md': {
		display: 'none',
	},
});

const BodyCopy = styled('div', {
	padding: '$10',
	display: 'flex',
	flexDirection: 'column',
	justifyContent: 'center',
	alignItems: 'center',
	flex: 1,
});

const SelectedWalletIcon = styled('img', {
	background: 'white',
	objectFit: 'cover',
	width: 72,
	height: 72,
	borderRadius: 16,
});

const ButtonContainer = styled('div', {
	position: 'absolute',
	bottom: '$8',
	right: '$8',
	marginTop: '$4',
});

const LeftPanel = styled('div', {
	display: 'flex',
	flexDirection: 'column',
	width: '100%',
	flex: 1,
	'@md': {
		flex: 0,
		minWidth: 240,
	},

	variants: {
		hasSelected: {
			true: {
				display: 'none',
				'@md': {
					display: 'block',
				},
			},
		},
	},
});

const OpeningWalletTitle = styled('div', {
	marginTop: '$3',
	marginBottom: '$1',
	color: '$textDark',
	fontSize: '$xl',
	fontWeight: '$title',
});

const ConnectionText = styled('div', {
	fontSize: '$xs',
	variants: {
		isError: {
			true: {
				color: '$issue',
			},
			false: {
				color: '$textLight',
			},
		},
	},
	defaultVariants: {
		isError: false,
	},
});

const MobileInfoButton = styled('button', {
	background: '$backgroundAccent',
	textAlign: 'center',
	width: '100%',
	padding: '$4',
	border: 'none',
	color: '$textLight',
	fontWeight: '$button',
	fontFamily: '$sans',
	cursor: 'pointer',

	'@md': {
		display: 'none',
	},
});

const SELECTED_INFO = '@@internal/what-is-wallet';

export function ConnectModal({ open, onClose, onError, text }: ConnectModalProps) {
	const { connect, currentWallet, isConnected, isError } = useWalletKit();
	const [selected, setSelected] = useState<string | null>(null);

	useEffect(() => {
		if (!open) {
			setSelected(null);
		}
	}, [open]);

	useEffect(() => {
		if (isConnected && currentWallet?.name === selected) {
			onClose();
		}
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [currentWallet, selected, isConnected]);

	return (
		<Dialog open={open} onClose={onClose}>
			<Overlay />
			<Content>
				<Body connect>
					<LeftPanel hasSelected={!!selected}>
						<WalletList
							selected={selected}
							onChange={(walletName) => {
								setSelected(walletName);
								connect(walletName)
									.catch(onError);
							}}
							{...text?.walletList}
						/>
						<MobileInfoButton onClick={() => setSelected(SELECTED_INFO)}>
							{text?.whatIsAWallet?.title || 'What is a Wallet'}
						</MobileInfoButton>
					</LeftPanel>

					<Panel responsiveHidden={!selected}>
						<BackButton onClick={() => setSelected(null)} aria-label="Back">
							<BackIcon />
						</BackButton>

						{!selected || selected === SELECTED_INFO ? (
							<>
								<Title css={{ textAlign: 'center' }}>
									{text?.whatIsAWallet?.title || 'What is a Wallet'}
								</Title>

								<BodyCopy>
									<WhatIsAWallet {...text?.whatIsAWallet} />
								</BodyCopy>
							</>
						) : selected && selected !== SELECTED_GETTING_STARTED ? (
							<BodyCopy>
								<SelectedWalletIcon src={currentWallet?.icon} />
								<OpeningWalletTitle>
									{text?.opening || 'Opening'} {selected}
								</OpeningWalletTitle>
								<ConnectionText isError={isError}>
									{isError
										? text?.connectionFailed || 'Connection failed'
										: text?.confirmConnection || 'Confirm connection in the wallet...'}
								</ConnectionText>

								{isError && (
									<ButtonContainer>
										<Button color="secondary" onClick={() => connect(selected)}>
											{text?.retryConnection || 'Retry Connection'}
										</Button>
									</ButtonContainer>
								)}
							</BodyCopy>
						) : (
							<>
								<Title css={{ textAlign: 'center' }}>
									{text?.getStarted || 'Get Started with BenFen'}{' '}
								</Title>

								<BodyCopy>
									<GettingStarted {...text?.gettingStarted} />
									<ButtonContainer>
										<Button
											as="a"
											color="secondary"
											href="https://openblock.com/#/download"
											target="_blank"
											rel="noopener noreferrer"
										>
											{text?.installWallet || 'Install Wallet Extension'}
										</Button>
									</ButtonContainer>
								</BodyCopy>
							</>
						)}
					</Panel>

					<CloseButton onClick={onClose} />
				</Body>
			</Content>
		</Dialog>
	);
}
