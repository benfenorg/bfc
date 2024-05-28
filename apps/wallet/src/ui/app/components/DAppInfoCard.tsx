// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { ArrowUpRight12 } from '@mysten/icons';
import { cx } from 'class-variance-authority';

import { AccountAddress } from './AccountAddress';
import { Link } from '../shared/Link';
import { Heading } from '../shared/heading';
import { Text } from '../shared/text';
=======
import { type PermissionType } from '_src/shared/messaging/messages/payloads/permissions';
>>>>>>> mainnet-v1.24.1
import { getValidDAppUrl } from '_src/shared/utils';
import { CheckFill16 } from '@mysten/icons';
import cn from 'clsx';

import { useAccountByAddress } from '../hooks/useAccountByAddress';
import { Heading } from '../shared/heading';
import { Link } from '../shared/Link';
import { AccountIcon } from './accounts/AccountIcon';
import { AccountItem } from './accounts/AccountItem';
import { LockUnlockButton } from './accounts/LockUnlockButton';
import { useUnlockAccount } from './accounts/UnlockAccountContext';
import { DAppPermissionsList } from './DAppPermissionsList';
import { SummaryCard } from './SummaryCard';

export type DAppInfoCardProps = {
	name: string;
	url: string;
	iconUrl?: string;
	connectedAddress?: string;
};

export function DAppInfoCard({ name, url, iconUrl, connectedAddress }: DAppInfoCardProps) {
	const validDAppUrl = getValidDAppUrl(url);
	const appHostname = validDAppUrl?.hostname ?? url;
	const { data: account } = useAccountByAddress(connectedAddress);
	const { unlockAccount, lockAccount, isPending, accountToUnlock } = useUnlockAccount();

	return (
<<<<<<< HEAD
		<div className="flex flex-col items-stretch p-2.5 pt-5 rounded-lg border border-solid border-bfc-border">
			<div className="flex flex-row flex-nowrap items-center gap-2.5 mb-2.5">
				<div className="flex items-stretch h-12 w-12 rounded-full overflow-hidden bg-[#d9d9d9] shrink-0 grow-0">
					{iconUrl ? <img className="flex-1" src={iconUrl} alt={name} /> : null}
				</div>
				<div className="flex flex-col flex-nowrap gap-1.5">
					<Heading variant="heading4" weight="semibold" color="bfc-text1">
						{name}
					</Heading>
					<Text variant="body" weight="normal" color="bfc-text3">
						{appHostname}
					</Text>
				</div>
			</div>
			{connectedAddress ? (
				<div className="p-2.5 flex flex-nowrap flex-row items-center gap-1.25">
					<Text variant="body" weight="medium" color="bfc-text1" truncate>
						Connected account
					</Text>
					<div className="flex-1" />
					<AccountAddress copyable address={connectedAddress} />
				</div>
			) : null}
			<div
				className={cx(
					'p-2.5 pb-0 flex justify-start border-0 border-t border-solid border-bfc-border',
				)}
			>
				<Link
					href={validDAppUrl?.toString() ?? url}
					title={name}
					size="body"
					text="View Website"
					after={<ArrowUpRight12 />}
					color="bfc-link"
					weight="medium"
				/>
			</div>
=======
		<div className="bg-white p-6 flex flex-col gap-5">
			<div className="flex flex-row flex-nowrap items-center gap-3.75 py-3">
				<div className="flex items-stretch h-15 w-15 overflow-hidden bg-steel/20 shrink-0 grow-0 rounded-2xl">
					{iconUrl ? <img className="flex-1" src={iconUrl} alt={name} /> : null}
				</div>
				<div className="flex flex-col items-start flex-nowrap gap-1 overflow-hidden">
					<div className="max-w-full overflow-hidden">
						<Heading variant="heading4" weight="semibold" color="gray-100" truncate>
							{name}
						</Heading>
					</div>
					<div className="max-w-full overflow-hidden">
						<Link
							href={validDAppUrl?.toString() ?? url}
							title={name}
							text={appHostname}
							color="heroDark"
							weight="medium"
						/>
					</div>
				</div>
			</div>
			{connectedAddress && account ? (
				<AccountItem
					icon={<AccountIcon account={account} />}
					accountID={account.id}
					disabled={account.isLocked}
					after={
						<div className="flex flex-1 items-center justify-end gap-1">
							{account.isLocked ? (
								<div className="h-4">
									<LockUnlockButton
										isLocked={account.isLocked}
										isLoading={isPending && accountToUnlock?.id === account.id}
										onClick={(e) => {
											// prevent the account from being selected when clicking the lock button
											e.stopPropagation();
											if (account.isLocked) {
												unlockAccount(account);
											} else {
												lockAccount(account);
											}
										}}
									/>
								</div>
							) : null}
							<CheckFill16
								className={cn('h-4 w-4', account.isLocked ? 'text-hero/10' : 'text-success')}
							/>
						</div>
					}
					hideCopy
					hideExplorerLink
				/>
			) : null}
			{permissions?.length ? (
				<SummaryCard
					header="Permissions requested"
					body={<DAppPermissionsList permissions={permissions} />}
					boxShadow
				/>
			) : null}
>>>>>>> mainnet-v1.24.1
		</div>
	);
}
