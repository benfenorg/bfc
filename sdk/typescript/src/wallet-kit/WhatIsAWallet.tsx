// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CopyContainer, Description, Heading } from './utils/ui.js';

export interface Props {
	subTitle1?: string;
	desc1?: string;
	subTitle2?: string;
	desc2?: string;
}

export function WhatIsAWallet({
	subTitle1 = 'Easy Login',
	desc1 = 'No need to create new accounts and passwords for every website. Just connect your wallet and get going.',
	subTitle2 = 'Store your Digital Assets',
	desc2 = 'Send, receive, store, and display your digital assets like NFTs & coins.',
}: Props) {
	return (
		<CopyContainer>
			<div>
				<Heading>{subTitle1}</Heading>
				<Description>{desc1}</Description>
			</div>

			<div>
				<Heading>{subTitle2}</Heading>
				<Description>{desc2}</Description>
			</div>
		</CopyContainer>
	);
}
