// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@benfen/bfc.js';
import { humanReadableToBfcDigits, strToHex } from '@benfen/bfc.js/utils';
import { useZodForm } from '@mysten/core';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@benfen/bfc.js';
import { useMutation } from '@tanstack/react-query';
import { useContext } from 'react';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { useDryRunTransactionBlock } from '~/hooks/useDryRunTransactionBlock';
import { Input } from '~/ui/Input';
import { ADDRESS } from '~/utils/constants';

export function CreateDaoAction() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { refetch, balance } = useContext(DaoContext)!;
	const dryRun = useDryRunTransactionBlock();

	const schema = z.object({
		amount: z
			.string()
			.transform(Number)
			.refine((n) => n >= 100, 'amount should be greater than or equal to 100')
			.refine(
				(n) => humanReadableToBfcDigits(n) <= BigInt(balance?.totalBalance || ''),
				'insufficient balance',
			),
		text: z.string().trim().min(1),
	});

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ text, amount }: { amount: number; text: string }) => {
			const bigIntAmount = humanReadableToBfcDigits(amount);

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

			await dryRun(tx);
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
			<Input label="text" maxLength={100} {...register('text')} />
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
