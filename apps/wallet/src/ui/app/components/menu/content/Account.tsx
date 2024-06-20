// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SerializedAccount } from '_src/background/keyring/Account';
import { useCopyToClipboard } from '_src/ui/app/hooks/useCopyToClipboard';
import { Link } from '_src/ui/app/shared/Link';
import { formatAddress } from '@benfen/bfc.js/utils';
import { Copy12 } from '@mysten/icons';
import { useEffect, useRef, useState } from 'react';

import { useNextMenuUrl } from '../hooks';

export type AccountProps = {
	account: SerializedAccount;
};

export function Account({ account }: AccountProps) {
	const [isActionsVisible, setIsActionsVisible] = useState(false);
	const overlayRef = useRef<HTMLDivElement>(null);
	const { address } = account;
	const copyCallback = useCopyToClipboard(address, {
		copySuccessMessage: 'Address copied',
	});

	const exportAccountUrl = useNextMenuUrl(true, `/export/${account.address}`);
	const recoveryPassphraseUrl = useNextMenuUrl(true, '/recovery-passphrase');

	useEffect(() => {
		if (isActionsVisible) {
			const onClick = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (overlayRef.current && !overlayRef.current.contains(target)) {
					setIsActionsVisible(false);
					document.removeEventListener('click', onClick);
				}
			};
			document.addEventListener('click', onClick);
			return () => {
				document.removeEventListener('click', onClick);
			};
		}
		return () => {};
	}, [isActionsVisible]);

	return (
		<div className="flex items-center gap-2.5">
			<div className="grow text-bodySmall font-medium text-bfc-text1">{formatAddress(address)}</div>
			<div
				className="w-[22px] h-[22px] rounded flex items-center justify-center cursor-pointer"
				onClick={copyCallback}
			>
				<Copy12 />
			</div>
			<div
				ref={overlayRef}
				className="relative w-[22px] h-[22px] rounded flex items-center justify-center cursor-pointer"
				onClick={(e) => {
					e.stopPropagation();
					setIsActionsVisible(true);
				}}
			>
				<svg
					width="14"
					height="14"
					viewBox="0 0 14 14"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<g clipPath="url(#clip0_377_5975)">
						<path
							d="M1.75 11.0832H12.25V12.2498H1.75V11.0832ZM7.58333 3.3995V9.9165H6.41667V3.3995L2.87525 6.9415L2.05042 6.11667L7 1.1665L11.9496 6.11609L11.1247 6.94092L7.58333 3.40067V3.3995Z"
							fill="#5A6070"
						/>
					</g>
					<defs>
						<clipPath id="clip0_377_5975">
							<rect width="14" height="14" fill="white" />
						</clipPath>
					</defs>
				</svg>
				{isActionsVisible && (
					<div className="absolute top-6 right-0 p-1 w-32 bg-white rounded-lg border border-solid border-bfc-border shadow-accountAction flex flex-col items-stretch z-10">
						<div className="h-[26px] rounded flex items-center text-bodySmall hover:bg-bfc-card">
							<Link
								text="Export Private Key"
								to={exportAccountUrl}
								color="bfc-text1"
								weight="normal"
							/>
						</div>
						<div className="h-[26px] rounded flex items-center text-bodySmall hover:bg-bfc-card">
							<Link
								to={recoveryPassphraseUrl}
								color="bfc-text1"
								weight="normal"
								text="Export Passphrase"
							/>
						</div>
					</div>
				)}
			</div>
		</div>
	);
}
