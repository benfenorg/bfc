// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { API_ENV_TO_INFO } from '_app/ApiProvider';
import { Button } from '_app/shared/ButtonUI';
import { lockWallet } from '_app/wallet/actions';
import { useNextMenuUrl } from '_components/menu/hooks';
import { useAppDispatch, useAppSelector } from '_hooks';
import { FAQ_LINK, ToS_LINK } from '_src/shared/constants';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import { useActiveAddress } from '_src/ui/app/hooks/useActiveAddress';
import { useAutoLockInterval } from '_src/ui/app/hooks/useAutoLockInterval';
import { useExplorerLink } from '_src/ui/app/hooks/useExplorerLink';
import { logout } from '_src/ui/app/redux/slices/account';
import { ConfirmationModal } from '_src/ui/app/shared/ConfirmationModal';
import FaucetRequestButton from '_src/ui/app/shared/faucet/FaucetRequestButton';
import { Link } from '_src/ui/app/shared/Link';
import { Text } from '_src/ui/app/shared/text';
import { formatAddress } from '@benfen/bfc.js/utils';
import { useResolveSuiNSName } from '@mysten/core';
import {
	Account24,
	ArrowUpRight12,
	ArrowUpRight16,
	CopyArchiveDoNotUse24,
	Domain24,
	Version24,
} from '@mysten/icons';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import Browser from 'webextension-polyfill';

import LoadingIndicator from '../../loading/LoadingIndicator';
import { MenuLayout } from './MenuLayout';
import MenuListItem from './MenuListItem';

function MenuList() {
	const accountUrl = useNextMenuUrl(true, '/accounts');
	const networkUrl = useNextMenuUrl(true, '/network');
	const autoLockUrl = useNextMenuUrl(true, '/auto-lock');
	const address = useActiveAddress();
	const { data: domainName } = useResolveSuiNSName(address);
	const apiEnv = useAppSelector((state) => state.app.apiEnv);
	const networkName = API_ENV_TO_INFO[apiEnv].name;
	const autoLockInterval = useAutoLockInterval();
	const version = Browser.runtime.getManifest().version;
	const dispatch = useAppDispatch();
	const navigate = useNavigate();
	const [logoutInProgress, setLogoutInProgress] = useState(false);
	const [isLogoutDialogOpen, setIsLogoutDialogOpen] = useState(false);
	const explorerAddress = useExplorerLink({
		type: ExplorerLinkType.address,
		address: address || '',
	});
	return (
		<>
			<MenuLayout title="Wallet Settings">
				<div className="flex flex-col divide-y divide-x-0 divide-solid divide-gray-45 border-b border-bfc-border">
					<div className="flex flex-col gap-2.5 mt-5 pt-5 px-2.5 pb-2.5 border-b border-bfc-border">
						<MenuListItem
							to={accountUrl}
							icon={<Account24 />}
							title={'Accounts'}
							subtitle={domainName ?? (address ? formatAddress(address) : '')}
						/>
						{explorerAddress && (
							<Button
								variant="secondary"
								size="narrow"
								href={explorerAddress}
								text={
									<Text variant="bodySmall" weight="medium" color="bfc-text2">
										View account on BenFen Explorer
									</Text>
								}
								after={<ArrowUpRight16 className="text-bfc-text2 w-3.5 h-3.5" />}
							/>
						)}
					</div>
					<div className="py-5 px-2.5 border-b border-bfc-border">
						<MenuListItem
							to={networkUrl}
							icon={<Domain24 />}
							title="Network"
							subtitle={networkName}
						/>
					</div>
					<div className="py-5 px-2.5 border-b border-bfc-border">
						<MenuListItem
							to={autoLockUrl}
							icon={<Version24 />}
							title="Auto-lock"
							subtitle={autoLockInterval ? `${autoLockInterval} min` : <LoadingIndicator />}
						/>
					</div>
					<div className="py-5 px-2.5 border-b border-bfc-border">
						<MenuListItem
							icon={<CopyArchiveDoNotUse24 />}
							title={
								<div className="flex gap-1.5 items-center">
									FAQ
									<ArrowUpRight12 className="text-steel w-3 h-3" />
								</div>
							}
							href={FAQ_LINK}
						/>
					</div>
				</div>
				<div className="flex flex-col items-stretch mt-2.5">
					<FaucetRequestButton variant="secondary" />
				</div>
				<div className="flex-1" />
				<div className="flex flex-nowrap flex-row items-stretch gap-3 mt-2.5">
					<div
						className="flex justify-center items-center h-10 w-full rounded-lg border border-solid border-bfc-border text-bodySmall font-medium text-bfc-text2 cursor-pointer"
						onClick={async () => {
							try {
								await dispatch(lockWallet()).unwrap();
								navigate('/locked', { replace: true });
							} catch (e) {
								// Do nothing
							}
						}}
					>
						Lock Wallet
					</div>
					<div
						className="flex justify-center items-center h-10 w-full rounded-lg border border-solid border-bfc-border text-bodySmall font-medium text-bfc-text2 cursor-pointer"
						onClick={async () => {
							if (logoutInProgress || isLogoutDialogOpen) {
								return;
							}
							setIsLogoutDialogOpen(true);
						}}
					>
						Logout
					</div>
				</div>
				<div className="px-2.5 flex flex-col items-center justify-center no-underline gap-3.75 mt-3.75">
					<Link
						href={ToS_LINK}
						text="Terms of service"
						after={
							<svg
								width="15"
								height="14"
								viewBox="0 0 15 14"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
							>
								<g clipPath="url(#clip0_15_4490)">
									<path
										d="M9.01084 4.66418L3.9895 9.68493L4.81434 10.5098L9.83508 5.4896V9.91418H11.0018V3.49752H4.58509V4.66418H9.01084Z"
										fill="#5A6070"
									/>
								</g>
								<defs>
									<clipPath id="clip0_15_4490">
										<rect width="14" height="14" fill="white" transform="matrix(1 0 0 -1 0.5 14)" />
									</clipPath>
								</defs>
							</svg>
						}
						color="bfc-text2"
						weight="normal"
					/>
					<Text variant="bodySmall" weight="normal" color="bfc-text3">
						Wallet Version v{version}
					</Text>
				</div>
			</MenuLayout>
			<ConfirmationModal
				isOpen={isLogoutDialogOpen}
				confirmText="Logout"
				confirmStyle="outlineWarning"
				title="Are you sure you want to Logout?"
				hint="You will need the 12-word Recovery Passphrase that was created when you first set up the wallet to log back in."
				onResponse={async (confirmed) => {
					setIsLogoutDialogOpen(false);
					if (confirmed) {
						setLogoutInProgress(true);
						try {
							await dispatch(logout());
							window.location.reload();
						} finally {
							setLogoutInProgress(false);
						}
					}
				}}
			/>
		</>
	);
}

export default MenuList;
