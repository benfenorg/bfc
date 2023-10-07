// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { type ProposalRecord } from '@mysten/sui.js/client';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useMemo } from 'react';
import { z } from 'zod';

import { useGetOBCDaoVote } from '~/hooks/useGetOBCDaoVote';
import { useZodForm } from '@mysten/core';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({
	vote: z.string().trim().nonempty(),
	agree: z.string().transform(Number),
});

export function ChangeVote({ proposal, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();

	const { data: votes = [], refetch: refetchVotes } = useGetOBCDaoVote(
		currentAccount?.address || '',
	);

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ vote, agree }: { agree: number; vote: string }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::obc_system::change_vote`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(vote),
					tx.object(proposal.proposal_uid),
					tx.pure(!!agree),
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
			refetchDao();
			refetchVotes();
		},
	});

	const options = useMemo(
		() =>
			votes
				.filter((i) => i.vid === proposal.pid.toString())
				.map((i) => ({
					label: i.id.id,
					value: i.id.id,
				})),
		[proposal, votes],
	);

	return (
		<form
			onSubmit={handleSubmit((formData) => {
				execute.mutateAsync(formData).catch((e) => {
					console.error(`failed to change vote`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Selector label="voting" options={options} {...register('vote')} />
			<Selector
				label="agree"
				options={[
					{ label: 'upvote', value: 1 },
					{ label: 'downvote', value: 0 },
				]}
				{...register('agree')}
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
