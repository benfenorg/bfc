// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
	DryRunTransactionBlockResponse,
	GasCostSummary,
	BenfenGasData,
	BenfenTransactionBlockResponse,
	TransactionEffects,
} from '@benfen/bfc.js/client';

type Optional<T> = {
	[K in keyof T]?: T[K];
};

export type GasSummaryType =
	| (GasCostSummary &
			Optional<BenfenGasData> & {
				totalGas?: string;
				owner?: string;
				isSponsored: boolean;
				gasUsed: GasCostSummary;
			})
	| null;

export function getGasSummary(
	transaction: BenfenTransactionBlockResponse | DryRunTransactionBlockResponse,
): GasSummaryType {
	const { effects } = transaction;
	if (!effects) return null;
	const totalGas = getTotalGasUsed(effects);

	let sender = 'transaction' in transaction ? transaction.transaction?.data.sender : undefined;

	const gasData = 'transaction' in transaction ? transaction.transaction?.data.gasData : {};

	const owner =
		'transaction' in transaction
			? transaction.transaction?.data.gasData.owner
			: typeof effects.gasObject.owner === 'object' && 'AddressOwner' in effects.gasObject.owner
			? effects.gasObject.owner.AddressOwner
			: '';

	return {
		...effects.gasUsed,
		...gasData,
		owner,
		totalGas: totalGas?.toString(),
		isSponsored: !!owner && !!sender && owner !== sender,
		gasUsed: transaction?.effects!.gasUsed,
	};
}

export function getTotalGasUsed(effects: TransactionEffects): bigint | undefined {
	const gasSummary = effects?.gasUsed;
	return gasSummary
		? BigInt(gasSummary.computationCost) +
				BigInt(gasSummary.storageCost) -
				BigInt(gasSummary.storageRebate)
		: undefined;
}
