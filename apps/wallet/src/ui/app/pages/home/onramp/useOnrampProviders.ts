// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';

import Banxa from './icons/Banxa.svg';
import MoonPay from './icons/MoonPay.svg';
import Transak from './icons/Transak.svg';
import { type OnrampProvider } from './types';

const TRANSAK_API_KEY =
	process.env.NODE_ENV === 'production'
		? '0318063c-0380-4e20-9c73-e08836240c17'
		: 'c72e867b-4069-4e22-a01e-4553353faefd';

const BACKEND_HOST =
	process.env.NODE_ENV === 'production' ? 'https://apps-backend.sui.io' : 'http://localhost:3003';

const ONRAMP_PROVIDER: OnrampProvider[] = [
	{
		key: 'banxa',
		name: 'Banxa',
		icon: Banxa,
		checkSupported: async () => {
			return false;
		},
		getUrl: async (address) => {
			const params = new URLSearchParams({
				coinType: 'SUI',
				fiatType: 'USD',
				fiatAmount: '100',
				blockchain: 'SUI',
				theme: 'dark',
				walletAddress: address,
				returnUrl: window.location.href,
			});
			const url = `https://suiwallet.banxa.com/?${params}`;
			return url;
		},
	},
	{
		key: 'transak',
		icon: Transak,
		name: 'Transak',
		checkSupported: async () => {
			return false;
		},
		getUrl: async (address) => {
			const params = new URLSearchParams({
				apiKey: TRANSAK_API_KEY,
				environment: process.env.NODE_ENV === 'production' ? 'PRODUCTION' : 'STAGING',
				// If you want to test ETH values, you can use something like this:
				// cryptoCurrencyCode: 'ETH',
				// walletAddress: '0x000000000000000000000000000000000000dead',
				cryptoCurrencyCode: 'SUI',
				walletAddress: address,
				disableWalletAddressForm: 'true',
				themeColor: '#6fbcf0',
			});

			return process.env.NODE_ENV === 'production'
				? `https://global.transak.com?${params}`
				: `https://global-stg.transak.com?${params}`;
		},
	},
	{
		key: 'moonpay',
		icon: MoonPay,
		name: 'MoonPay',
		checkSupported: async () => {
			return false;
		},
		getUrl: async (address) => {
			const params = new URLSearchParams({
				theme: 'light',
				colorCode: '#6fbcf0',
				currencyCode: 'SUI',
				walletAddress: address,
				environment: process.env.NODE_ENV === 'production' ? 'PRODUCTION' : 'STAGING',
			});

			const res = await fetch(`${BACKEND_HOST}/moonpay-url?${params}`);

			const data = (await res.json()) as { url: string };

			return data.url;
		},
	},
];

const PREFERRED_ONRAMP_PROVIDER_KEY = 'preferred-onramp-provider';

export function useOnrampProviders() {
	const [preferredProviderKey, setPreferredProviderKey] = useState(() =>
		localStorage.getItem(PREFERRED_ONRAMP_PROVIDER_KEY),
	);

	const { data } = useQuery({
		queryKey: ['onramp', 'get-providers'],
		queryFn: async () => {
			const supportedProviders = await Promise.all(
				ONRAMP_PROVIDER.map(async (provider) => {
					const supported = await provider.checkSupported();
					return supported ? provider.key : null;
				}),
			);

			// NOTE: We don't put the actual provider instances into the cache, because that will fail when persisting.
			// Instead, we use a selector to get the actual provider instances from the keys.
			return supportedProviders.filter(Boolean) as string[];
		},
		enabled: false,
		select(providerKeys) {
			const providers = providerKeys
				.map((key) => ONRAMP_PROVIDER.find((provider) => provider.key === key))
				.filter(Boolean) as OnrampProvider[];

			if (!preferredProviderKey) {
				return providers;
			}

			const preferredProvider = providers.find(({ key }) => key === preferredProviderKey);
			const nonPreferredProviders = providers.filter(({ key }) => key !== preferredProviderKey);

			return [preferredProvider, ...nonPreferredProviders].filter(Boolean) as OnrampProvider[];
		},
	});

	return {
		providers: data,
		setPreferredProvider(key: string) {
			localStorage.setItem(PREFERRED_ONRAMP_PROVIDER_KEY, key);
			setPreferredProviderKey(key);
		},
	};
}
