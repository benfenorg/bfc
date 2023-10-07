// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { type ProposalRecord, ProposalStatus } from '@mysten/sui.js/client';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useMemo } from 'react';
import { z } from 'zod';

import { useZodForm } from '@mysten/core';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({
	status: z.string().transform(Number),
});

export function ModifyProposalObj({ proposal, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();

	const { handleSubmit, formState, register } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ status }: { status: number }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::obc_system::modify_proposal`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(proposal.proposal_uid),
					tx.pure(status),
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
		},
	});

	const options = useMemo(() => {
		const codes = Object.values(ProposalStatus).filter((i) => typeof i === 'number') as number[];
		return codes.map((i) => ({ label: ProposalStatus[i], value: i }));
	}, []);

	return (
		<form
			onSubmit={handleSubmit((formData) => {
				execute.mutateAsync(formData).catch((e) => {
					console.error(`failed to modify proposal`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Selector label="status" options={options} {...register('status')} />
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
