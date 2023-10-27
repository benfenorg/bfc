// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactNode, useState } from 'react';
import { ConnectModal } from '@mysten/wallet-kit';
import { Button } from './Base/Button';

interface ConnectExcuteButtonProps {
	currentAccount?: any;
	connectedText?: ReactNode;
	disabled?: boolean;
	excute?: any;
}

export function ConnectExcuteButton({
    currentAccount,
	connectedText,
	disabled,
	excute,
}: ConnectExcuteButtonProps) {
	const [connectModalOpen, setConnectModalOpen] = useState(false);

	return (
		<>
			{currentAccount ? (
				<Button onClick={excute} disabled={disabled}>
					{connectedText}
				</Button>
			) : (
				<Button onClick={() => setConnectModalOpen(true)}>Connect Wallet</Button>
			)}

			{!currentAccount && (
				<ConnectModal open={connectModalOpen} onClose={() => setConnectModalOpen(false)} />
			)}
		</>
	);
}
