// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useCallback } from 'react';
import { Navigate, Route, Routes, useLocation, useNavigate } from 'react-router-dom';

import { AccountsSettings } from './AccountsSettings';
import { AutoLockSettings } from './AutoLockSettings';
import { ExportAccount } from './ExportAccount';
import { ImportPrivateKey } from './ImportPrivateKey';
import MenuList from './MenuList';
import { NetworkSettings } from './NetworkSettings';
import { ConnectLedgerModalContainer } from '../../ledger/ConnectLedgerModalContainer';
=======
>>>>>>> mainnet-v1.24.1
import { ErrorBoundary } from '_components/error-boundary';
import {
	MainLocationContext,
	useMenuIsOpen,
	useMenuUrl,
	useNextMenuUrl,
} from '_components/menu/hooks';
import { RecoveryPassphrase } from '_components/recovery-passphrase/RecoveryPassphrase';
import { useOnKeyboardEvent } from '_hooks';
<<<<<<< HEAD

import { ImportLedgerAccountsPage } from '_src/ui/app/pages/accounts/ImportLedgerAccountsPage';
=======
import { useCallback } from 'react';
>>>>>>> mainnet-v1.24.1
import type { MouseEvent } from 'react';
import { Navigate, Route, Routes, useLocation, useNavigate } from 'react-router-dom';

import { AutoLockAccounts } from './AutoLockAccounts';
import { MoreOptions } from './MoreOptions';
import { NetworkSettings } from './NetworkSettings';
import WalletSettingsMenuList from './WalletSettingsMenuList';

const CLOSE_KEY_CODES: string[] = ['Escape'];

function MenuContent() {
	const mainLocation = useLocation();
	const isOpen = useMenuIsOpen();
	const menuUrl = useMenuUrl();
	const menuHomeUrl = useNextMenuUrl(true, '/');
	const closeMenuUrl = useNextMenuUrl(false);
	const navigate = useNavigate();
	const handleOnCloseMenu = useCallback(
		(e: KeyboardEvent | MouseEvent<HTMLDivElement>) => {
			if (isOpen) {
				e.preventDefault();
				navigate(closeMenuUrl);
			}
		},
		[isOpen, navigate, closeMenuUrl],
	);
	useOnKeyboardEvent('keydown', CLOSE_KEY_CODES, handleOnCloseMenu, isOpen);
	if (!isOpen) {
		return null;
	}

	return (
<<<<<<< HEAD
		<div className="absolute flex flex-col justify-items-stretch inset-0 bg-white pb-8 px-5 pt-5 z-50 rounded-xl overflow-y-auto">
=======
		<div className="absolute flex flex-col justify-items-stretch inset-0 bg-white pb-8 px-2.5 z-50 rounded-t-xl overflow-y-auto">
>>>>>>> mainnet-v1.24.1
			<ErrorBoundary>
				<MainLocationContext.Provider value={mainLocation}>
					<Routes location={menuUrl || ''}>
						<Route path="/" element={<MenuList />} />
						<Route path="/accounts" element={<AccountsSettings />}>
							<Route path="connect-ledger-modal" element={<ConnectLedgerModalContainer />} />
						</Route>
						<Route path="/export/:account" element={<ExportAccount />} />
						<Route path="/import-private-key" element={<ImportPrivateKey />} />
						<Route path="/network" element={<NetworkSettings />} />
<<<<<<< HEAD
						<Route path="/auto-lock" element={<AutoLockSettings />} />
=======
						<Route path="/auto-lock" element={<AutoLockAccounts />} />
						<Route path="/more-options" element={<MoreOptions />} />
>>>>>>> mainnet-v1.24.1
						<Route path="*" element={<Navigate to={menuHomeUrl} replace={true} />} />
						<Route path="/import-ledger-accounts" element={<ImportLedgerAccountsPage />} />
						<Route path="/recovery-passphrase" element={<RecoveryPassphrase />} />
					</Routes>
				</MainLocationContext.Provider>
			</ErrorBoundary>
		</div>
	);
}

export default MenuContent;
