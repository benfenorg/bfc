// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@benfen/bfc.js';
import { humanReadableToBfcDigits } from '@benfen/bfc.js/utils';
import { useWalletKit } from '@benfen/wallet-kit';
import { useZodForm } from '@mysten/core';
import { Button } from '@mysten/ui';
import { useMutation } from '@tanstack/react-query';
import { useContext, useMemo } from 'react';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { useDryRunTransactionBlock } from '~/hooks/useDryRunTransactionBlock';
import { Input } from '~/ui/Input';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({
	vote: z.string({ required_error: 'must select voting' }).trim().nonempty(),
	amount: z
		.string({
			required_error: 'amount must be a valid number',
			invalid_type_error: 'amount must be a valid number',
		})
		.transform(Number)
		.refine((n) => n >= 1, 'amount must be greater than or equal to 1'),
});

export function RevokeVote() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { proposal, votes, refetch } = useContext(DaoContext)!;
	const dryRun = useDryRunTransactionBlock();

	const options = useMemo(
		() =>
			votes
				.filter((i) => i.vid === proposal!.pid.toString())
				.map((i) => ({
					label: i.id.id,
					value: i.id.id,
				})),
		[proposal, votes],
	);

	const { handleSubmit, register, formState, control } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ amount, vote }: { amount: number; vote: string }) => {
			const bigIntAmount = humanReadableToBfcDigits(amount);

			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::revoke_vote`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.BFC_SYSTEM_STATE),
					tx.object(proposal!.proposal_uid),
					tx.object(vote),
					tx.pure(bigIntAmount),
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
					console.error(`failed to revoke vote`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Controller
				control={control}
				name="vote"
				render={({ field: { value, onChange } }) => (
					<Selector label="voting" options={options} value={value} onChange={onChange} />
				)}
			/>
			<Input label="amount" type="number" step="any" {...register('amount')} />
			<div className="flex items-stretch gap-1.5">
				<Button
					variant="primary"
					type="submit"
					loading={execute.isLoading}
					disabled={!isConnected || options.length === 0}
				>
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
