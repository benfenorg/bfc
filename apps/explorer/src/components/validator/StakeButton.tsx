// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { Button } from '@mysten/ui';
import { ConnectButton, useWalletKit, type WalletWithFeatures } from '@benfen/wallet-kit';
import { useParams } from 'react-router-dom';

// This is a custom feature supported by the Sui Wallet:
type StakeInput = { validatorAddress: string };
type SuiWalletStakeFeature = {
	'benfenWallet:stake': {
		version: '0.0.1';
		stake: (input: StakeInput) => Promise<void>;
	};
};

type StakeWallet = WalletWithFeatures<Partial<SuiWalletStakeFeature>>;

export function StakeButton() {
	const stakeButtonEnabled = useFeatureIsOn('validator-page-staking');
	const { id } = useParams();
	const { wallets, currentWallet, connect } = useWalletKit();

	if (!stakeButtonEnabled) return null;

	const stakeSupportedWallets = wallets.filter((wallet) => {
		const standardWallet = wallet as StakeWallet;
		return 'benfenWallet:stake' in standardWallet.features;
	});

	const currentWalletSupportsStake =
		currentWallet && !!stakeSupportedWallets.find(({ name }) => currentWallet.name === name);

	if (!stakeSupportedWallets.length) {
		return (
			<Button size="lg" asChild>
				<a href="https://openblock.com/#/download" target="_blank" rel="noreferrer noopener">
					Install BenFen Wallet to stake BFC
				</a>
			</Button>
		);
	}

	if (!currentWallet) {
		return (
			<ConnectButton
				className="!border !border-solid !border-steel-dark !bg-transparent !px-4 !py-3 !text-body !font-semibold !text-steel-dark !shadow-none"
				connectText="Stake BFC"
			/>
		);
	}

	if (!currentWalletSupportsStake) {
		return (
			<Button
				size="lg"
				onClick={() => {
					// Always just assume we should connect to the first stake supported wallet for now:
					connect(stakeSupportedWallets[0].name);
				}}
			>
				Stake BFC on a supported wallet
			</Button>
		);
	}

	return (
		<Button
			size="lg"
			onClick={() => {
				(currentWallet as StakeWallet).features['benfenWallet:stake']?.stake({
					validatorAddress: id!,
				});
			}}
		>
			Stake BFC
		</Button>
	);
}
