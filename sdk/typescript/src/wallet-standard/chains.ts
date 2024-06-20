// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import type { IdentifierString } from '@wallet-standard/core';

/** Sui Devnet */
export const BFC_DEVNET_CHAIN = 'bfc:devnet';

/** Sui Testnet */
export const BFC_TESTNET_CHAIN = 'bfc:testnet';

/** Sui Localnet */
export const BFC_LOCALNET_CHAIN = 'bfc:localnet';

/** Sui Mainnet */
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
 * Utility that returns whether or not a chain identifier is a valid Sui chain.
 * @param chain a chain identifier in the form of `${string}:{$string}`
 */
export function isBfcChain(chain: IdentifierString): chain is BfcChain {
	return BFC_CHAINS.includes(chain as BfcChain);
}
