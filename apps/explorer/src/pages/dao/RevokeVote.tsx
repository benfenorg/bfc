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
import { humanReadableToObcDigits } from '@mysten/sui.js/utils';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useMemo } from 'react';
import { z } from 'zod';

import { useGetOBCDaoVote } from '~/hooks/useGetOBCDaoVote';
import { Input } from '~/ui/Input';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({
	vote: z.string().trim().nonempty(),
	amount: z.string().transform(Number),
});

export function RevokeVote({ proposal, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();

	const { data: votes = [], refetch: refetchVotes } = useGetOBCDaoVote(
		currentAccount?.address || '',
	);

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ amount, vote }: { amount: number; vote: string }) => {
			const bigIntAmount = humanReadableToObcDigits(amount);

			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::obc_system::revoke_vote`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(proposal.proposal_uid),
					tx.object(vote),
					tx.pure(bigIntAmount),
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
					console.error(`failed to revoke vote`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Selector label="voting" options={options} {...register('vote')} />
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
