// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useNextMenuUrl } from '_components/menu/hooks';
import { useAccounts } from '_src/ui/app/hooks/useAccounts';
import { useDeriveNextAccountMutation } from '_src/ui/app/hooks/useDeriveNextAccountMutation';
import { Button } from '_src/ui/app/shared/ButtonUI';
import React from 'react';
import { Outlet } from 'react-router-dom';

import { Account } from './Account';
import { MenuLayout } from './MenuLayout';

export function AccountsSettings() {
	const backUrl = useNextMenuUrl(true, '/');
	const importPrivateKeyUrl = useNextMenuUrl(true, '/import-private-key');
	const accounts = useAccounts();
	const createAccountMutation = useDeriveNextAccountMutation();

	return (
		<MenuLayout title="Accounts" back={backUrl}>
			<div className="flex flex-col gap-3">
				<div className="flex flex-col justify-center items-stretch gap-2.5 p-2.5 border border-solid border-bfc-border rounded-xl">
					{accounts.map((account, index) => (
						<React.Fragment key={account.address}>
							<Account key={account.address} account={account} />
							{index !== accounts.length - 1 && (
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="280"
									height="2"
									viewBox="0 0 280 2"
									fill="none"
								>
									<path d="M0 1H280" stroke="#E1E1E9" />
								</svg>
							)}
						</React.Fragment>
					))}
				</div>
				<Button
					variant="account"
					size="tall"
					text="Create New Account"
					loading={createAccountMutation.isPending}
					onClick={() => createAccountMutation.mutate()}
					before={
						<svg
							width="14"
							height="14"
							viewBox="0 0 15 14"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M6.9165 6.4165V2.9165H8.08317V6.4165H11.5832V7.58317H8.08317V11.0832H6.9165V7.58317H3.4165V6.4165H6.9165Z"
								fill="#171719"
							/>
						</svg>
					}
				/>
				<Button
					variant="account"
					size="tall"
					text="Import Private Key"
					to={importPrivateKeyUrl}
					before={
						<svg
							width="14"
							height="14"
							viewBox="0 0 14 14"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M1.75 11.0832H12.25V12.2498H1.75V11.0832ZM7.58333 7.6835L11.1248 4.1415L11.9496 4.96634L7 9.9165L2.05042 4.96692L2.87525 4.1415L6.41667 7.68234V1.1665H7.58333V7.6835Z"
								fill="#171719"
							/>
						</svg>
					}
				/>
				<Outlet />
			</div>
		</MenuLayout>
	);
}
