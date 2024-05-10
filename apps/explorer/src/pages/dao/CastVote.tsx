// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@benfen/bfc.js';
import { useZodForm } from '@mysten/core';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@benfen/bfc.js';
import { useMutation } from '@tanstack/react-query';
import BigNumber from 'bignumber.js';
import { useContext } from 'react';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { useDryRunTransactionBlock } from '~/hooks/useDryRunTransactionBlock';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({
	voting: z.string({ required_error: 'must select voting' }).trim().nonempty(),
	agree: z.number({ required_error: 'must select agree' }),
});

export function CastVote() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();

	const { votingBfcs, proposal, refetch } = useContext(DaoContext)!;
	const dryRun = useDryRunTransactionBlock();

	const { handleSubmit, formState, control } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ voting, agree }: { voting: string; agree: number }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::cast_vote`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.BFC_SYSTEM_STATE),
					tx.object(proposal!.proposal_uid),
					tx.object(voting),
					tx.pure(!!agree),
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
					console.error(`failed to cast vote`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Controller
				control={control}
				name="voting"
				render={({ field: { value, onChange } }) => (
					<Selector
						label="voting"
						options={votingBfcs.map((i) => ({
							label: new BigNumber(i.principal).shiftedBy(-9).toString(),
							value: i.id.id,
						}))}
						value={value}
						onChange={onChange}
					/>
				)}
			/>
			<Controller
				control={control}
				name="agree"
				render={({ field: { value, onChange } }) => (
					<Selector
						label="agree"
						options={[
							{ label: 'upvote', value: 1 },
							{ label: 'downvote', value: 0 },
						]}
						value={value}
						onChange={onChange}
					/>
				)}
			/>

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
