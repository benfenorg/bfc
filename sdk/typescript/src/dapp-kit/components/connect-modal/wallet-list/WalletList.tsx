// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { WalletWithRequiredFeatures } from '../../../../wallet-standard/index.js';
import { useWallets } from '../../../hooks/wallet/useWallets.js';
import { getWalletUniqueIdentifier } from '../../../utils/walletUtils.js';
import { BenfenIcon } from '../../icons/BenfenIcon.js';
import * as styles from './WalletList.css.js';
import { WalletListItem } from './WalletListItem.js';

type WalletListProps = {
	selectedWalletName?: string;
	onPlaceholderClick: () => void;
	onSelect: (wallet: WalletWithRequiredFeatures) => void;
};

export function WalletList({ selectedWalletName, onPlaceholderClick, onSelect }: WalletListProps) {
	const wallets = useWallets();
	return (
		<ul className={styles.container}>
			{wallets.length > 0 ? (
				wallets.map((wallet) => (
					<WalletListItem
						key={getWalletUniqueIdentifier(wallet)}
						name={wallet.name}
						icon={wallet.icon}
						isSelected={getWalletUniqueIdentifier(wallet) === selectedWalletName}
						onClick={() => onSelect(wallet)}
					/>
				))
			) : (
				<WalletListItem
					name="OpenBlock Wallet"
					icon={<BenfenIcon />}
					onClick={onPlaceholderClick}
					isSelected
				/>
			)}
		</ul>
	);
}
