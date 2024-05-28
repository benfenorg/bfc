// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useCallback, useEffect, useMemo, useState } from 'react';
import { useParams } from 'react-router-dom';

import { DAppPermissionsList } from '../../components/DAppPermissionsList';
import { SummaryCard } from '../../components/SummaryCard';
import { WalletListSelect } from '../../components/WalletListSelect';
import { useActiveAddress } from '../../hooks/useActiveAddress';
import { PageMainLayoutTitle } from '../../shared/page-main-layout/PageMainLayoutTitle';
=======
import { AccountItemApproveConnection } from '_components/accounts/AccountItemApproveConnection';
>>>>>>> mainnet-v1.24.1
import Loading from '_components/loading';
import { UserApproveContainer } from '_components/user-approve-container';
import { useAppDispatch, useAppSelector } from '_hooks';
import type { RootState } from '_redux/RootReducer';
import { permissionsSelectors, respondToPermissionRequest } from '_redux/slices/permissions';
<<<<<<< HEAD
import { ampli } from '_src/shared/analytics/ampli';

import type { RootState } from '_redux/RootReducer';
=======
import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { ampli } from '_src/shared/analytics/ampli';
import { useCallback, useEffect, useMemo, useState } from 'react';
import { useParams } from 'react-router-dom';
>>>>>>> mainnet-v1.24.1

import { AccountMultiSelectWithControls } from '../../components/accounts/AccountMultiSelect';
import Alert from '../../components/alert';
import { SectionHeader } from '../../components/SectionHeader';
import { useAccountGroups } from '../../hooks/useAccountGroups';
import { useActiveAccount } from '../../hooks/useActiveAccount';
import { PageMainLayoutTitle } from '../../shared/page-main-layout/PageMainLayoutTitle';
import st from './SiteConnectPage.module.scss';

function SiteConnectPage() {
	const { requestID } = useParams();
	const permissionsInitialized = useAppSelector(({ permissions }) => permissions.initialized);
	const loading = !permissionsInitialized;
	const permissionSelector = useMemo(
		() => (state: RootState) =>
			requestID ? permissionsSelectors.selectById(state, requestID) : null,
		[requestID],
	);
	const dispatch = useAppDispatch();
	const permissionRequest = useAppSelector(permissionSelector);
<<<<<<< HEAD
	const activeAddress = useActiveAddress();
	const [accountsToConnect, setAccountsToConnect] = useState<string[]>(() =>
		activeAddress ? [activeAddress] : [],
=======
	const activeAccount = useActiveAccount();
	const accountGroups = useAccountGroups();
	const accounts = accountGroups.list();
	const unlockedAccounts = accounts.filter((account) => !account.isLocked);
	const lockedAccounts = accounts.filter((account) => account.isLocked);
	const [accountsToConnect, setAccountsToConnect] = useState<SerializedUIAccount[]>(() =>
		activeAccount && !activeAccount.isLocked ? [activeAccount] : [],
>>>>>>> mainnet-v1.24.1
	);
	const handleOnSubmit = useCallback(
		async (allowed: boolean) => {
			if (requestID && accountsToConnect && permissionRequest) {
				await dispatch(
					respondToPermissionRequest({
						id: requestID,
						accounts: allowed ? accountsToConnect : [],
						allowed,
					}),
				);
				ampli.respondedToConnectionRequest({
					applicationName: permissionRequest.name,
					applicationUrl: permissionRequest.origin,
					approvedConnection: allowed,
				});
				window.close();
			}
		},
		[requestID, accountsToConnect, permissionRequest, dispatch],
	);
	useEffect(() => {
		if (!loading && !permissionRequest) {
			window.close();
		}
	}, [loading, permissionRequest]);

	const parsedOrigin = useMemo(
		() => (permissionRequest ? new URL(permissionRequest.origin) : null),
		[permissionRequest],
	);

	const isSecure = parsedOrigin?.protocol === 'https:';
	const [displayWarning, setDisplayWarning] = useState(!isSecure);

	const handleHideWarning = useCallback(
		async (allowed: boolean) => {
			if (allowed) {
				setDisplayWarning(false);
			} else {
				await handleOnSubmit(false);
			}
		},
		[handleOnSubmit],
	);

	useEffect(() => {
		setDisplayWarning(!isSecure);
	}, [isSecure]);
	return (
		<Loading loading={loading}>
			{permissionRequest &&
				(displayWarning ? (
					<UserApproveContainer
						origin={permissionRequest.origin}
						originFavIcon={permissionRequest.favIcon}
						approveTitle="Continue"
						rejectTitle="Reject"
						onSubmit={handleHideWarning}
						isWarning
						addressHidden
						blended
					>
						<PageMainLayoutTitle title="Insecure Website" />
						<div>
							<div className={st.warningWrapper}>
								<span className={st.warningTitle}>Your Connection is Not Secure</span>
							</div>
							<div className={st.warningMessage}>
								If you connect your wallet to this site your data could be exposed to attackers.
								Click **Reject** if you don't trust this site.
								<br />
								<br />
								Continue at your own risk.
							</div>
						</div>
					</UserApproveContainer>
				) : (
					<UserApproveContainer
						origin={permissionRequest.origin}
						originFavIcon={permissionRequest.favIcon}
						approveTitle="Connect"
						rejectTitle="Reject"
						onSubmit={handleOnSubmit}
						approveDisabled={!accountsToConnect.length}
						blended
					>
						<PageMainLayoutTitle title="Approve Connection" />
<<<<<<< HEAD
						<SummaryCard
							header="Permissions requested"
							body={<DAppPermissionsList permissions={permissionRequest.permissions} />}
						/>
						<div className="w-full h-5"></div>
						<WalletListSelect
							title="Connect Accounts"
							values={accountsToConnect}
							onChange={setAccountsToConnect}
						/>
=======
						<div className="flex flex-col gap-8 py-6">
							{unlockedAccounts.length > 0 ? (
								<AccountMultiSelectWithControls
									selectedAccountIDs={accountsToConnect.map((account) => account.id)}
									accounts={unlockedAccounts ?? []}
									onChange={(value) => {
										setAccountsToConnect(value.map((id) => accounts?.find((a) => a.id === id)!));
									}}
								/>
							) : (
								<Alert mode="warning">
									All accounts are currently locked. Unlock accounts to connect.
								</Alert>
							)}
							{lockedAccounts?.length > 0 && (
								<div className="flex flex-col gap-3">
									<SectionHeader title="Locked & Unavailable" />
									{lockedAccounts?.map((account) => (
										<AccountItemApproveConnection
											key={account.id}
											showLock
											account={account}
											disabled={account.isLocked}
										/>
									))}
								</div>
							)}
						</div>
>>>>>>> mainnet-v1.24.1
					</UserApproveContainer>
				))}
		</Loading>
	);
}

export default SiteConnectPage;
