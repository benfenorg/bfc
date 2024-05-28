// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { getTransactionKind, getTransactionSender } from '@benfen/bfc.js';
import { type SuiTransactionBlockResponse } from '@benfen/bfc.js/client';
=======
import { getAmount } from '_helpers';
import { type SuiTransactionBlockResponse } from '@mysten/sui.js/client';
>>>>>>> mainnet-v1.24.1
import { useMemo } from 'react';

type Props = {
	txn: SuiTransactionBlockResponse;
	address: string;
};

export function useGetTxnRecipientAddress({ txn, address }: Props) {
	const events = txn.events!;

	// const eventsSummary = useMemo(() => {
	//     const { coins } = getEventsSummary(events, address);
	//     return coins;
	// }, [events, address]);

	const transaction = getTransactionKind(txn)!;
	const amountByRecipient = getAmount(transaction, txn.effects!, events);

	const recipientAddress = useMemo(() => {
		const transferObjectRecipientAddress =
			amountByRecipient &&
			amountByRecipient?.find(({ recipientAddress }) => recipientAddress !== address)
				?.recipientAddress;
		// MUSTFIX(chris)
		// const receiverAddr =
		//     eventsSummary &&
		//     eventsSummary.find(
		//         ({ receiverAddress }) => receiverAddress !== address
		//     )?.receiverAddress;

		return null ?? transferObjectRecipientAddress ?? getTransactionSender(txn);
	}, [address, amountByRecipient, txn]);
	// }, [address, amountByRecipient, eventsSummary, txn]);

	return recipientAddress;
}
