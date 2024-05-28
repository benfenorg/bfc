// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { type SuiClient } from '@benfen/bfc.js/client';
=======
import { createSuiAddressValidation } from '_components/address-input/validation';
import { type SuiClient } from '@mysten/sui.js/client';
>>>>>>> mainnet-v1.24.1
import * as Yup from 'yup';

export function createValidationSchema(
	client: SuiClient,
	suiNSEnabled: boolean,
	senderAddress: string,
	objectId: string,
) {
	return Yup.object({
		to: createSuiAddressValidation(client, suiNSEnabled)
			.test(
				'sender-address',
				// eslint-disable-next-line no-template-curly-in-string
				`NFT is owned by this address`,
				(value) => senderAddress !== value,
			)
			.test(
				'nft-sender-address',
				// eslint-disable-next-line no-template-curly-in-string
				`NFT address must be different from receiver address`,
				(value) => objectId !== value,
			),
	});
}
