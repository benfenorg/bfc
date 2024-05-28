// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import cl from 'classnames';
import { useCallback, useMemo, useState } from 'react';

import { Button } from '../../shared/ButtonUI';
import { DAppInfoCard } from '../DAppInfoCard';

import type { ReactNode } from 'react';

import st from './UserApproveContainer.module.scss';
=======
import { type PermissionType } from '_src/shared/messaging/messages/payloads/permissions';
import cn from 'clsx';
import { useCallback, useMemo, useState } from 'react';
import type { ReactNode } from 'react';

import { useAccountByAddress } from '../../hooks/useAccountByAddress';
import { Button } from '../../shared/ButtonUI';
import { UnlockAccountButton } from '../accounts/UnlockAccountButton';
import { DAppInfoCard } from '../DAppInfoCard';
>>>>>>> mainnet-v1.24.1

type UserApproveContainerProps = {
	children: ReactNode | ReactNode[];
	origin: string;
	originFavIcon?: string;
	rejectTitle: string;
	approveTitle: string;
	approveDisabled?: boolean;
	approveLoading?: boolean;
	onSubmit: (approved: boolean) => Promise<void>;
	isWarning?: boolean;
	addressHidden?: boolean;
	address?: string | null;
	scrollable?: boolean;
	blended?: boolean;
<<<<<<< HEAD
=======
	permissions?: PermissionType[];
	checkAccountLock?: boolean;
>>>>>>> mainnet-v1.24.1
};

export function UserApproveContainer({
	origin,
	originFavIcon,
	children,
	rejectTitle,
	approveTitle,
	approveDisabled = false,
	approveLoading = false,
	onSubmit,
	isWarning,
	addressHidden = false,
	address,
<<<<<<< HEAD
	scrollable,
	blended = false,
=======
	permissions,
	checkAccountLock,
>>>>>>> mainnet-v1.24.1
}: UserApproveContainerProps) {
	const [submitting, setSubmitting] = useState(false);
	const handleOnResponse = useCallback(
		async (allowed: boolean) => {
			setSubmitting(true);
			await onSubmit(allowed);
			setSubmitting(false);
		},
		[onSubmit],
	);
	const { data: selectedAccount } = useAccountByAddress(address);
	const parsedOrigin = useMemo(() => new URL(origin), [origin]);
	return (
		<div className={st.container}>
			<div className={cl(st.scrollBody, { [st.scrollable]: scrollable })}>
				<DAppInfoCard
					name={parsedOrigin.host}
					url={origin}
					iconUrl={originFavIcon}
					connectedAddress={!addressHidden && address ? address : undefined}
				/>
<<<<<<< HEAD
				<div className={cl(st.children, { [st.scrollable]: scrollable })}>{children}</div>
			</div>
			<div className={st.actionsContainer}>
				<div className={cl(st.actions, isWarning && st.flipActions)}>
					<Button
						size="tall"
						variant="warning"
						onClick={() => {
							handleOnResponse(false);
						}}
						disabled={submitting}
						text={rejectTitle}
						before={
							<svg
								xmlns="http://www.w3.org/2000/svg"
								width="15"
								height="14"
								viewBox="0 0 15 14"
								fill="none"
							>
								<path
									d="M7.49993 6.17461L10.3874 3.28711L11.2123 4.11194L8.32476 6.99944L11.2123 9.88694L10.3874 10.7118L7.49993 7.82428L4.61243 10.7118L3.7876 9.88694L6.6751 6.99944L3.7876 4.11194L4.61243 3.28711L7.49993 6.17461Z"
									fill="#EB362A"
								/>
							</svg>
						}
					/>
					<Button
						// recreate the button when changing the variant to avoid animating to the new styles
						key={`approve_${isWarning}`}
						size="tall"
						variant={isWarning ? 'secondary' : 'primary'}
						onClick={() => {
							handleOnResponse(true);
						}}
						disabled={approveDisabled}
						loading={submitting || approveLoading}
						text={approveTitle}
					/>
=======
				<div className="flex flex-1 flex-col px-6 bg-hero-darkest/5">{children}</div>
			</div>
			<div className="sticky bottom-0">
				<div
					className={cn('bg-hero-darkest/5 backdrop-blur-lg py-4 px-5 flex items-center gap-2.5', {
						'flex-row-reverse': isWarning,
					})}
				>
					{!checkAccountLock || !selectedAccount?.isLocked ? (
						<>
							<Button
								size="tall"
								variant="secondary"
								onClick={() => {
									handleOnResponse(false);
								}}
								disabled={submitting}
								text={rejectTitle}
							/>
							<Button
								size="tall"
								variant={isWarning ? 'secondary' : 'primary'}
								onClick={() => {
									handleOnResponse(true);
								}}
								disabled={approveDisabled}
								loading={submitting || approveLoading}
								text={approveTitle}
							/>
						</>
					) : (
						<UnlockAccountButton account={selectedAccount} title="Unlock to Approve" />
					)}
>>>>>>> mainnet-v1.24.1
				</div>
			</div>
		</div>
	);
}
