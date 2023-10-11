// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ReactNode, useState } from 'react';
import { WalletAccount } from '@mysten/wallet-standard';
import { ConnectModal } from '@mysten/wallet-kit';
import { Button } from './Button';
interface ExcuteButtonProps {
	connectText?: ReactNode;
	connectedText?: ReactNode;
	currentAccount?: WalletAccount | null;
}


export function ExcuteButton({
	connectText = 'Connect Wallet',
	connectedText,
	currentAccount,
	...props
}: ExcuteButtonProps) {
	const [connectModalOpen, setConnectModalOpen] = useState(false);

	return (
		<>
			{currentAccount ? (
				<Button onClick={()=>{}}>
					铸造
				</Button>
			) : (
				<button
					className="bg-white text-bf-text1 px-3 py-2 rounded-md font-semibold"
					onClick={() => setConnectModalOpen(true)}
				>
					{connectText}
				</button>
			)}

			{!currentAccount && (
				<ConnectModal open={connectModalOpen} onClose={() => setConnectModalOpen(false)} />
			)}
		</>
	);
}
