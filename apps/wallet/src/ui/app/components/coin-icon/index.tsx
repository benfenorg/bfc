// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SUI_TYPE_ARG } from '@benfen/bfc.js';
=======
import { ImageIcon } from '_app/shared/image-icon';
>>>>>>> mainnet-v1.24.1
import { useCoinMetadata } from '@mysten/core';
import { Logo, Unstaked } from '@mysten/icons';
import { cva, type VariantProps } from 'class-variance-authority';

<<<<<<< HEAD
import { ImageIcon } from '_app/shared/image-icon';

const imageStyle = cva(['rounded-full flex rounded-full flex justify-center items-center'], {
=======
const imageStyle = cva(['rounded-full flex'], {
>>>>>>> mainnet-v1.24.1
	variants: {
		size: {
			sm: 'w-6 h-6',
			md: 'w-7.5 h-7.5',
			lg: 'md:w-10 md:h-10 w-8 h-8',
			xl: 'md:w-31.5 md:h-31.5 w-16 h-16 ',
		},
	},

	defaultVariants: {
		size: 'md',
	},
});

function SuiCoin() {
<<<<<<< HEAD
	return <Logo className="w-6 h-6 text-body rounded-full bg-bfc" />;
=======
	return (
		<Sui className="flex items-center w-full h-full justify-center text-white p-1.5 text-body rounded-full" />
	);
>>>>>>> mainnet-v1.24.1
}

type NonSuiCoinProps = {
	coinType: string;
};

function NonSuiCoin({ coinType }: NonSuiCoinProps) {
	const { data: coinMeta } = useCoinMetadata(coinType);
	return (
		<div className="flex h-full w-full items-center justify-center text-bfc rounded-full">
			{coinMeta?.iconUrl ? (
				<ImageIcon
					src={coinMeta.iconUrl}
					label={coinMeta.name || coinType}
					fallback={coinMeta.name || coinType}
					rounded="full"
				/>
			) : (
				<Unstaked />
			)}
		</div>
	);
}

export interface CoinIconProps extends VariantProps<typeof imageStyle> {
	coinType: string;
}

export function CoinIcon({ coinType, ...styleProps }: CoinIconProps) {
	return (
		<div className={imageStyle(styleProps)}>
			{coinType === SUI_TYPE_ARG ? <SuiCoin /> : <NonSuiCoin coinType={coinType} />}
		</div>
	);
}
