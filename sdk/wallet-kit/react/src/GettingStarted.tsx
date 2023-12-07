// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CopyContainer, Description, Heading } from './utils/ui';

export interface Props {
	title1?: string;
	desc1?: string;
	title2?: string;
	desc2?: string;
	title3?: string;
	desc3?: string;
}

export function GettingStarted({ title1, title2, title3, desc1, desc2, desc3 }: Props) {
	return (
		<CopyContainer>
			<div>
				<Heading>{title1 || 'Install the BenFen extension'}</Heading>
				<Description>
					{desc1 || 'We recommend pinning the BenFen Wallet to your taskbar for quicker access.'}
				</Description>
			</div>

			<div>
				<Heading>{title2 || 'Create or Import a Wallet'}</Heading>
				<Description>
					{desc2 ||
						'Be sure to back up your wallet using a secure method. Never share your secret phrase with anyone.'}
				</Description>
			</div>

			<div>
				<Heading>{title3 || 'Refresh Your Browser'}</Heading>
				<Description>
					{desc3 ||
						'Once you set up your wallet, refresh this window browser to load up the extension.'}
				</Description>
			</div>
		</CopyContainer>
	);
}
