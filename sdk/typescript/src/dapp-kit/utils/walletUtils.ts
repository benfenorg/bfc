// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { getWallets, isWalletWithRequiredFeatureSet } from '../../wallet-standard/index.js';
import type {
	MinimallyRequiredFeatures,
	Wallet,
	WalletWithFeatures,
} from '../../wallet-standard/index.js';

export {} from '@wallet-standard/core'; // fix ts error

export function getRegisteredWallets<AdditionalFeatures extends Wallet['features']>(
	preferredWallets: string[],
	requiredFeatures?: (keyof AdditionalFeatures)[],
) {
	const walletsApi = getWallets();
	const wallets = walletsApi.get();

	const benfenWallets = wallets.filter(
		(wallet): wallet is WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures> =>
			isWalletWithRequiredFeatureSet(wallet, requiredFeatures),
	);

	return [
		// Preferred wallets, in order:
		...(preferredWallets
			.map((name) => benfenWallets.find((wallet) => wallet.name === name))
			.filter(Boolean) as WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures>[]),

		// Wallets in default order:
		...benfenWallets.filter((wallet) => !preferredWallets.includes(wallet.name)),
	];
}

export function getWalletUniqueIdentifier(wallet?: Wallet) {
	return wallet?.id ?? wallet?.name;
}
