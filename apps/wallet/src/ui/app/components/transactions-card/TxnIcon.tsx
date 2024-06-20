// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	Account24,
	ArrowTopRight24,
	Info16,
	Sui,
	Swap16,
	Unstaked,
	WalletActionStake24,
} from '@mysten/icons';
import cl from 'classnames';

import LoadingIndicator from '../loading/LoadingIndicator';

const icons = {
	Send: <ArrowTopRight24 className="text-bfc text-body" />,
	Receive: <ArrowTopRight24 className="text-bfc text-body rotate-180" />,
	Transaction: <ArrowTopRight24 className="text-bfc text-body" />,
	Staked: <WalletActionStake24 className="text-gradient-blue-start text-heading2 bg-transparent" />,
	Unstaked: <Unstaked className="text-gradient-blue-start text-heading3" />,
	Rewards: <Sui className="text-gradient-blue-start text-body" />,
	Swapped: <Swap16 className="text-gradient-blue-start text-heading6" />,
	Failed: <Info16 className="text-issue-dark text-heading6" />,
	Loading: <LoadingIndicator />,
	PersonalMessage: <Account24 fill="currentColor" className="text-gradient-blue-start text-body" />,
};

interface TxnItemIconProps {
	txnFailed?: boolean;
	variant: keyof typeof icons;
}

export function TxnIcon({ txnFailed, variant }: TxnItemIconProps) {
	return (
		<div className={cl(['w-6 h-6 flex justify-center items-center'])}>
			{icons[txnFailed ? 'Failed' : variant]}
		</div>
	);
}
