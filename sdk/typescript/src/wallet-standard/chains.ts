// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0
import type { IdentifierString } from '@wallet-standard/core';

/** Benfen Devnet */
export const BFC_DEVNET_CHAIN = 'bfc:devnet';

/** Benfen Testnet */
export const BFC_TESTNET_CHAIN = 'bfc:testnet';

/** Benfen Localnet */
export const BFC_LOCALNET_CHAIN = 'bfc:localnet';

/** Benfen Mainnet */
export const BFC_MAINNET_CHAIN = 'bfc:mainnet';

export const BFC_CHAINS = [
	BFC_DEVNET_CHAIN,
	BFC_TESTNET_CHAIN,
	BFC_LOCALNET_CHAIN,
	BFC_MAINNET_CHAIN,
] as const;

export type BfcChain =
	| typeof BFC_DEVNET_CHAIN
	| typeof BFC_TESTNET_CHAIN
	| typeof BFC_LOCALNET_CHAIN
	| typeof BFC_MAINNET_CHAIN;

/**
 * Utility that returns whether or not a chain identifier is a valid Benfen chain.
 * @param chain a chain identifier in the form of `${string}:{$string}`
 */
export function isBfcChain(chain: IdentifierString): chain is BfcChain {
	return BFC_CHAINS.includes(chain as BfcChain);
}
