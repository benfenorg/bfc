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
import BigNumber from 'bignumber.js';
import { z } from 'zod';

import { useGetOBCDaoVotingObc } from '~/hooks/useGetOBCDaoVotingObc';
import { useZodForm } from '~/hooks/useZodForm';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({
	voting: z.string().trim().nonempty(),
	agree: z.string().transform(Number),
});

export function CastVote({ proposal, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();

	const { data: votingObcs = [], refetch: refetchVoting } = useGetOBCDaoVotingObc(
		currentAccount?.address || '',
	);

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ voting, agree }: { voting: string; agree: number }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::obc_system::cast_vote`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(proposal.proposal_uid),
					tx.object(voting),
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
			refetchVoting();
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
			<Selector
				label="voting"
				options={votingObcs.map((i) => ({
					label: new BigNumber(i.principal).shiftedBy(-9).toString(),
					value: i.id.id,
				}))}
				{...register('voting')}
			/>
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
