// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { getWallets, isWalletWithRequiredFeatureSet } from '../../wallet-standard/index.js';
import type {
	MinimallyRequiredFeatures,
	StandardConnectFeature,
	StandardEventsFeature,
	Wallet,
	WalletWithFeatures,
	WalletWithRequiredFeatures,
} from '../../wallet-standard/index.js';

export function getRegisteredWallets<AdditionalFeatures extends Wallet['features']>(
	preferredWallets: string[],
	walletFilter?: (wallet: WalletWithRequiredFeatures) => boolean,
): WalletWithFeatures<StandardConnectFeature & StandardEventsFeature & AdditionalFeatures>[] {
	const walletsApi = getWallets();
	const wallets = walletsApi.get();

	const bfcWallets = wallets.filter(
		(wallet): wallet is WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures> =>
			isWalletWithRequiredFeatureSet(wallet) && (!walletFilter || walletFilter(wallet)),
	);

	return [
		// Preferred wallets, in order:
		...(preferredWallets
			.map((name) => bfcWallets.find((wallet) => wallet.name === name))
			.filter(Boolean) as WalletWithFeatures<MinimallyRequiredFeatures & AdditionalFeatures>[]),

		// Wallets in default order:
		...bfcWallets.filter((wallet) => !preferredWallets.includes(wallet.name)),
	];
}

export function getWalletUniqueIdentifier(wallet?: Wallet) {
	return wallet?.id ?? wallet?.name;
}
