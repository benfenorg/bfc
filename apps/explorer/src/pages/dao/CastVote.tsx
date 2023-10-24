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
import BigNumber from 'bignumber.js';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { useGetBFCDaoVotingBfc } from '~/hooks/useGetBFCDaoVotingBfc';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({
	voting: z.string().trim().nonempty('must select one voting'),
	agree: z.number(),
});

export function CastVote({ proposal, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();

	const { data: votingBfcs = [], refetch: refetchVoting } = useGetBFCDaoVotingBfc(
		currentAccount?.address || '',
	);

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
