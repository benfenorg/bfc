// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SerializedAccount } from '_src/background/keyring/Account';
import { formatAddress, sui2BfcAddress } from '@benfen/bfc.js/utils';
import { useResolveSuiNSName } from '@mysten/core';
import { Check24, Copy12 } from '@mysten/icons';

import { useActiveAddress } from '../hooks/useActiveAddress';
import { useCopyToClipboard } from '../hooks/useCopyToClipboard';
import { Text } from '../shared/text';
import { AccountBadge } from './AccountBadge';

export type AccountItemProps = {
	account: SerializedAccount;
	onAccountSelected: (address: SerializedAccount) => void;
};

export function AccountListItem({ account, onAccountSelected }: AccountItemProps) {
	const { address, type } = account;
	const activeAddress = useActiveAddress();
	const copy = useCopyToClipboard(sui2BfcAddress(address), {
		copySuccessMessage: 'Address Copied',
	});
	const { data: domainName } = useResolveSuiNSName(address);

	return (
		<li>
			<button
				className="appearance-none bg-transparent border-0 w-full flex gap-2.5 px-2.5 py-2 items-center hover:bg-bfc-card cursor-pointer group text-left"
				onClick={() => {
					onAccountSelected(account);
				}}
			>
				<div className="flex items-center gap-2 flex-1 min-w-0">
					<div className="min-w-0">
						<Text color="bfc-text1" variant="body" truncate mono>
							{domainName ?? formatAddress(address)}
						</Text>
					</div>
					<AccountBadge accountType={type} />
				</div>
				{activeAddress === address ? <Check24 className="text-bfc w-3.5 h-3.5" /> : null}
				<Copy12 className="text-bfc" onClick={copy} />
			</button>
		</li>
	);
}
