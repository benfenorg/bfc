// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useZodForm } from '@mysten/core';
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
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { useGetBFCDaoVote } from '~/hooks/useGetBFCDaoVote';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({
	vote: z.string().trim().nonempty(),
});

export function UnvoteVotes({ proposal, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();

	const { data: votes = [], refetch: refetchVotes } = useGetBFCDaoVote(
		currentAccount?.address || '',
	);

	const { handleSubmit, formState, control } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ vote }: { vote: string }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::unvote_votes`,
				typeArguments: [],
				arguments: [tx.object(proposal.proposal_uid), tx.object(vote), tx.object(ADDRESS.CLOCK)],
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
					console.error(`failed to unvote votes`, e);
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
