// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { BenfenClient, BenfenObjectResponse } from '@benfen/bfc.js/client';

// get NFT's owner from RPC.
export const getOwner = async (client: BenfenClient, nftId: string): Promise<string | null> => {
	const ownerResponse = await client.getObject({
		id: nftId,
		options: { showOwner: true },
	});
	const owner = ownerResponse.data?.owner;
	return (
		(owner as { AddressOwner: string })?.AddressOwner ||
		(owner as { ObjectOwner: string })?.ObjectOwner ||
		null
	);
};

// get avatar NFT Object from RPC.
export const getAvatar = async (
	client: BenfenClient,
	avatar: string,
): Promise<BenfenObjectResponse> => {
	return await client.getObject({
		id: avatar,
		options: {
			showDisplay: true,
			showOwner: true,
		},
	});
};
