// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useZodForm } from '@mysten/core';
import { useSuiClient } from '@mysten/dapp-kit';
import {
	type CoinStruct,
	SUI_TYPE_ARG,
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { humanReadableToBfcDigits, strToHex } from '@mysten/sui.js/utils';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useContext } from 'react';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { Input } from '~/ui/Input';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({
	amount: z
		.string()
		.transform(Number)
		.refine((n) => n >= 100, 'amount should be greater than or equal to 100'),
	text: z.string().trim().min(1),
});

export function CreateDaoAction() {
	const client = useSuiClient();

	const { isConnected, currentAccount, signAndExecuteTransactionBlock } = useWalletKit();
	const { refetch } = useContext(DaoContext)!;

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ text, amount }: { amount: number; text: string }) => {
			const bigIntAmount = humanReadableToBfcDigits(amount);

			const allCoins: CoinStruct[] = [];
			let cursor: string | undefined = undefined;
			for (;;) {
				const temp = await client.getCoins({
					owner: currentAccount!.address,
					coinType: SUI_TYPE_ARG,
					cursor,
				});
				allCoins.push(...temp.data);
				if (!temp.nextCursor) {
					break;
				}
				cursor = temp.nextCursor;
			}
			// TODO: 检查是否需要 mergeCoins

			// if (BigInt(largest.balance) < bigIntAmount) {
			// 	const tx = new TransactionBlock();

			// 	tx.mergeCoins(
			// 		tx.object(allCoins[0].coinObjectId),
			// 		allCoins.slice(1).map((i) => tx.object(i.coinObjectId)),
			// 	);

			// 	const result = await signAndExecuteTransactionBlock({ transactionBlock: tx });
			// 	if (getExecutionStatusType(result) === 'failure') {
			// 		throw new Error(getExecutionStatusError(result) || 'Transaction failed');
			// 	}
			// } else {
			// 	gas = tx.object(largest.coinObjectId);
			// }

			const tx = new TransactionBlock();

			const coin = tx.splitCoins(tx.gas, [tx.pure(bigIntAmount)]);
			tx.moveCall({
				target: `0xc8::bfc_system::create_bfcdao_action`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.BFC_SYSTEM_STATE),
					coin,
					tx.object(strToHex(text)),
					tx.object(ADDRESS.CLOCK),
				],
			});

			const result = await signAndExecuteTransactionBlock({
				transactionBlock: tx,
			});
			if (getExecutionStatusType(result) === 'failure') {
				throw new Error(getExecutionStatusError(result) || 'Transaction failed');
			}
			return result;
		},
		onSuccess: () => {
			refetch();
		},
	});
	return (
		<form
			onSubmit={handleSubmit((formData) => {
				execute.mutateAsync(formData).catch((e) => {
					console.error(`failed to create dao action`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Input label="amount" type="number" step="any" {...register('amount')} />
			<Input label="text" {...register('text')} />
			<div className="flex items-stretch gap-1.5">
				<Button variant="primary" type="submit" loading={execute.isLoading} disabled={!isConnected}>
					execute
				</Button>
			</div>
			{Object.values(formState.errors).length > 0
				? Object.keys(formState.errors).map((i) => (
						<div key={i}>{formState.errors[i as keyof typeof formState.errors]?.message}</div>
				  ))
				: null}
			{execute.error ? <div className="">{(execute.error as Error).message || 'Error'}</div> : null}
			{execute.data && <div>{getTransactionDigest(execute.data)}</div>}
		</form>
	);
}
