// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

<<<<<<<< HEAD:sdk/typescript/src/zksend/wallet.ts
import type { Emitter } from 'mitt';
import mitt from 'mitt';

import { bcs } from '../bcs/index.js';
import { toB64 } from '../utils/index.js';
import { BFC_MAINNET_CHAIN, getWallets, ReadonlyWalletAccount } from '../wallet-standard/index.js';
========
import { Transaction } from '@mysten/sui/transactions';
import { toB64 } from '@mysten/sui/utils';
>>>>>>>> releases/sui-v1.31.0-release:sdk/zksend/src/wallet/index.ts
import type {
	BenfenSignPersonalMessageFeature,
	BenfenSignPersonalMessageMethod,
	BenfenSignTransactionBlockFeature,
	BenfenSignTransactionBlockMethod,
	StandardConnectFeature,
	StandardConnectMethod,
	StandardDisconnectFeature,
	StandardDisconnectMethod,
	StandardEventsFeature,
	StandardEventsListeners,
	StandardEventsOnMethod,
<<<<<<<< HEAD:sdk/typescript/src/zksend/wallet.ts
	Wallet,
} from '../wallet-standard/index.js';
import { DEFAULT_ZKSEND_ORIGIN, ZkSendPopup } from './channel/index.js';
========
	SuiSignPersonalMessageFeature,
	SuiSignPersonalMessageMethod,
	SuiSignTransactionBlockFeature,
	SuiSignTransactionBlockMethod,
	SuiSignTransactionFeature,
	SuiSignTransactionMethod,
	Wallet,
} from '@mysten/wallet-standard';
import { getWallets, ReadonlyWalletAccount, SUI_MAINNET_CHAIN } from '@mysten/wallet-standard';
import type { Emitter } from 'mitt';
import mitt from 'mitt';

import { DEFAULT_STASHED_ORIGIN, StashedPopup } from './channel/index.js';
>>>>>>>> releases/sui-v1.31.0-release:sdk/zksend/src/wallet/index.ts

type WalletEventsMap = {
	[E in keyof StandardEventsListeners]: Parameters<StandardEventsListeners[E]>[0];
};

const STASHED_RECENT_ADDRESS_KEY = 'stashed:recentAddress';

export const STASHED_WALLET_NAME = 'Stashed' as const;

export class StashedWallet implements Wallet {
	#events: Emitter<WalletEventsMap>;
	#accounts: ReadonlyWalletAccount[];
	#origin: string;
	#name: string;

	get name() {
		return STASHED_WALLET_NAME;
	}

	get icon() {
		return 'data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1NiIgaGVpZ2h0PSI1NiIgZmlsbD0ibm9uZSI+PHJlY3Qgd2lkdGg9IjU0IiBoZWlnaHQ9IjU0IiB4PSIxIiB5PSIxIiBmaWxsPSIjNTE5REU5IiByeD0iMjciLz48cmVjdCB3aWR0aD0iNTQiIGhlaWdodD0iNTQiIHg9IjEiIHk9IjEiIHN0cm9rZT0iIzAwMCIgc3Ryb2tlLXdpZHRoPSIyIiByeD0iMjciLz48cGF0aCBmaWxsPSIjMDAwIiBkPSJNMTguMzUzIDM1LjA2NGMuOTIxIDMuNDM4IDQuMzYzIDYuNTUxIDExLjQ4MyA0LjY0NCA2Ljc5NC0xLjgyMSAxMS4wNTItNy40MSA5Ljk0OC0xMS41My0uMzgxLTEuNDIzLTEuNTMtMi4zODctMy4zLTIuMjNsLTE1LjgzMiAxLjMyYy0uOTk3LjA3Ni0xLjQ1NC0uMDg4LTEuNzE4LS43MTYtLjI1Ni0uNTk5LS4xMS0xLjI0MSAxLjA5NC0xLjg1bDEyLjA0OC02LjE4M2MuOTI0LS40NyAxLjUzOS0uNjY2IDIuMTAxLS40NjguMzUyLjEyOC41ODQuNjM4LjM3MSAxLjI2N2wtLjc4MSAyLjMwNmMtLjk1OSAyLjgzIDEuMDk0IDMuNDg4IDIuMjUgMy4xNzggMS43NTEtLjQ2OSAyLjE2My0yLjEzNiAxLjU5OS00LjI0LTEuNDMtNS4zMzctNy4wOS02LjE3LTEyLjIyMy00Ljc5Ni01LjIyMiAxLjQtOS43NDggNS42My04LjM2NiAxMC43ODkuMzI1IDEuMjE1IDEuNDQ0IDIuMTg2IDIuNzQgMi4xNTdsMS45NzgtLjAwNWMuNDA3LS4wMS4yNi4wMjQgMS4wNDYtLjA0MS43ODQtLjA2NSAyLjg4LS4zMjMgMi44OC0uMzIzbDEwLjI4Ni0xLjE2NC4yNjUtLjAzOGMuNjAyLS4xMDMgMS4wNTYuMDUzIDEuNDQuNzE1LjU3Ni45OTEtLjMwMiAxLjczOC0xLjM1MiAyLjYzM2wtLjA4NS4wNzItOS4wNDEgNy43OTJjLTEuNTUgMS4zMzctMi42MzkuODM0LTMuMDItLjU4OWwtMS4zNS01LjA0Yy0uMzM0LTEuMjQ0LTEuNTUtMi4yMjEtMi45NzQtMS44NC0xLjc4LjQ3Ny0xLjkyNCAyLjU1LTEuNDg3IDQuMThaIi8+PC9zdmc+Cg==' as const;
	}

	get version() {
		return '1.0.0' as const;
	}

	get chains() {
		return [BFC_MAINNET_CHAIN] as const;
	}

	get accounts() {
		return this.#accounts;
	}

	get features(): StandardConnectFeature &
		StandardDisconnectFeature &
		StandardEventsFeature &
<<<<<<<< HEAD:sdk/typescript/src/zksend/wallet.ts
		BenfenSignTransactionBlockFeature &
		BenfenSignPersonalMessageFeature {
========
		SuiSignTransactionBlockFeature &
		SuiSignTransactionFeature &
		SuiSignPersonalMessageFeature {
>>>>>>>> releases/sui-v1.31.0-release:sdk/zksend/src/wallet/index.ts
		return {
			'standard:connect': {
				version: '1.0.0',
				connect: this.#connect,
			},
			'standard:disconnect': {
				version: '1.0.0',
				disconnect: this.#disconnect,
			},
			'standard:events': {
				version: '1.0.0',
				on: this.#on,
			},
			'bfc:signTransactionBlock': {
				version: '1.0.0',
				signTransactionBlock: this.#signTransactionBlock,
			},
<<<<<<<< HEAD:sdk/typescript/src/zksend/wallet.ts
			'bfc:signPersonalMessage': {
========
			'sui:signTransaction': {
				version: '2.0.0',
				signTransaction: this.#signTransaction,
			},
			'sui:signPersonalMessage': {
>>>>>>>> releases/sui-v1.31.0-release:sdk/zksend/src/wallet/index.ts
				version: '1.0.0',
				signPersonalMessage: this.#signPersonalMessage,
			},
		};
	}

	constructor({
		name,
		address,
		origin = DEFAULT_STASHED_ORIGIN,
	}: {
		origin?: string;
		address?: string | null;
		name: string;
	}) {
		this.#accounts = [];
		this.#events = mitt();
		this.#origin = origin;
		this.#name = name;

		if (address) {
			this.#setAccount(address);
		}
	}

	#signTransactionBlock: BenfenSignTransactionBlockMethod = async ({
		transactionBlock,
		account,
	}) => {
		transactionBlock.setSenderIfNotSet(account.address);

		const data = transactionBlock.serialize();

		const popup = new StashedPopup({
			name: this.#name,
			origin: this.#origin,
		});

		const response = await popup.send({
			type: 'sign-transaction-block',
			data,
			address: account.address,
		});

		return {
			transactionBlockBytes: response.bytes,
			signature: response.signature,
		};
	};

<<<<<<<< HEAD:sdk/typescript/src/zksend/wallet.ts
	#signPersonalMessage: BenfenSignPersonalMessageMethod = async ({ message, account }) => {
		const bytes = toB64(bcs.vector(bcs.u8()).serialize(message).toBytes());
		const popup = new ZkSendPopup({ name: this.#name, origin: this.#origin });
		const response = await popup.createRequest({
========
	#signTransaction: SuiSignTransactionMethod = async ({ transaction, account }) => {
		const popup = new StashedPopup({
			name: this.#name,
			origin: this.#origin,
		});

		const tx = Transaction.from(await transaction.toJSON());
		tx.setSenderIfNotSet(account.address);

		const data = tx.serialize();

		const response = await popup.send({
			type: 'sign-transaction-block',
			data,
			address: account.address,
		});

		return {
			bytes: response.bytes,
			signature: response.signature,
		};
	};

	#signPersonalMessage: SuiSignPersonalMessageMethod = async ({ message, account }) => {
		const popup = new StashedPopup({
			name: this.#name,
			origin: this.#origin,
		});
		const bytes = toB64(message);

		const response = await popup.send({
>>>>>>>> releases/sui-v1.31.0-release:sdk/zksend/src/wallet/index.ts
			type: 'sign-personal-message',
			bytes,
			address: account.address,
		});

		return {
			bytes,
			signature: response.signature,
		};
	};

	#on: StandardEventsOnMethod = (event, listener) => {
		this.#events.on(event, listener);
		return () => this.#events.off(event, listener);
	};

	#setAccount(address?: string) {
		if (address) {
			this.#accounts = [
				new ReadonlyWalletAccount({
					address,
<<<<<<<< HEAD:sdk/typescript/src/zksend/wallet.ts
					chains: [BFC_MAINNET_CHAIN],
					features: ['bfc:signTransactionBlock', 'bfc:signPersonalMessage'],
					// NOTE: zkSend doesn't support getting public keys, and zkLogin accounts don't have meaningful public keys anyway
========
					chains: [SUI_MAINNET_CHAIN],
					features: ['sui:signTransactionBlock', 'sui:signPersonalMessage'],
					// NOTE: Stashed doesn't support getting public keys, and zkLogin accounts don't have meaningful public keys anyway
>>>>>>>> releases/sui-v1.31.0-release:sdk/zksend/src/wallet/index.ts
					publicKey: new Uint8Array(),
				}),
			];

			localStorage.setItem(STASHED_RECENT_ADDRESS_KEY, address);
		} else {
			this.#accounts = [];
		}

		this.#events.emit('change', { accounts: this.accounts });
	}

	#connect: StandardConnectMethod = async (input) => {
		if (input?.silent) {
			const address = localStorage.getItem(STASHED_RECENT_ADDRESS_KEY);

			if (address) {
				this.#setAccount(address);
			}

			return { accounts: this.accounts };
		}

		const popup = new StashedPopup({ name: this.#name, origin: this.#origin });

		const response = await popup.send({
			type: 'connect',
		});

		if (!('address' in response)) {
			throw new Error('Unexpected response');
		}

		this.#setAccount(response.address);

		return { accounts: this.accounts };
	};

	#disconnect: StandardDisconnectMethod = async () => {
		localStorage.removeItem(STASHED_RECENT_ADDRESS_KEY);
		this.#setAccount();
	};
}

export function registerStashedWallet(
	name: string,
	{
		origin,
	}: {
		origin?: string;
	},
) {
	const wallets = getWallets();

	let addressFromRedirect: string | null = null;
	try {
		const params = new URLSearchParams(window.location.search);
		addressFromRedirect = params.get('stashed_address') || params.get('zksend_address');
	} catch {
		// Ignore errors
	}

	const wallet = new StashedWallet({
		name,
		origin,
		address: addressFromRedirect,
	});

	const unregister = wallets.register(wallet);

	return {
		wallet,
		unregister,
		addressFromRedirect,
	};
}
