// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@benfen/bfc.js';
import { humanReadableToBfcDigits } from '@benfen/bfc.js/utils';
import { useZodForm } from '@mysten/core';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@benfen/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useContext } from 'react';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { useDryRunTransactionBlock } from '~/hooks/useDryRunTransactionBlock';
import { Input } from '~/ui/Input';
import { ADDRESS } from '~/utils/constants';

export function CreateVotingBfc() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { refetch, balance } = useContext(DaoContext)!;
	const dryRun = useDryRunTransactionBlock();

	const schema = z.object({
		amount: z
			.string()
			.transform(Number)
			.refine((n) => n >= 1, 'amount should be greater than or equal to 1')
			.refine(
				(n) => humanReadableToBfcDigits(n) <= BigInt(balance?.totalBalance || ''),
				'insufficient balance',
			),
	});

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ amount }: { amount: number }) => {
			const bigIntAmount = humanReadableToBfcDigits(amount);

			const tx = new TransactionBlock();
			const coin = tx.splitCoins(tx.gas, [tx.pure(bigIntAmount)]);

			tx.moveCall({
				target: `0xc8::bfc_system::create_voting_bfc`,
				typeArguments: [],
				arguments: [tx.object(ADDRESS.BFC_SYSTEM_STATE), coin, tx.object(ADDRESS.CLOCK)],
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
					console.error(`failed to create voting bfc`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Input label="amount" type="number" {...register('amount')} />
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
