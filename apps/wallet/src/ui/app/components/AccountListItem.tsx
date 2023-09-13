// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useResolveSuiNSName } from '@mysten/core';
import { Check24, Copy12 } from '@mysten/icons';
import { formatAddress, sui2ObcAddress } from '@mysten/sui.js';

import { AccountBadge } from './AccountBadge';
import { useActiveAddress } from '../hooks/useActiveAddress';
import { useCopyToClipboard } from '../hooks/useCopyToClipboard';
import { Text } from '../shared/text';
import { type SerializedAccount } from '_src/background/keyring/Account';

export type AccountItemProps = {
	account: SerializedAccount;
	onAccountSelected: (address: SerializedAccount) => void;
};

export function AccountListItem({ account, onAccountSelected }: AccountItemProps) {
	const { address, type } = account;
	const activeAddress = useActiveAddress();
	const copy = useCopyToClipboard(sui2ObcAddress(address), {
		copySuccessMessage: 'Address Copied',
	});
	const { data: domainName } = useResolveSuiNSName(address);

	return (
		<li>
			<button
				className="appearance-none bg-transparent border-0 w-full flex gap-2.5 px-2.5 py-2 items-center hover:bg-obc-card cursor-pointer group text-left"
				onClick={() => {
					onAccountSelected(account);
				}}
			>
				<div className="flex items-center gap-2 flex-1 min-w-0">
					<div className="min-w-0">
						<Text color="obc-text1" variant="body" truncate mono>
							{domainName ?? formatAddress(address)}
						</Text>
					</div>
					<AccountBadge accountType={type} />
				</div>
				{activeAddress === address ? <Check24 className="text-obc w-3.5 h-3.5" /> : null}
				<Copy12 className="text-obc" onClick={copy} />
			</button>
		</li>
	);
}
