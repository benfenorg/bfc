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
import { z } from 'zod';

import { useZodForm } from '@mysten/core';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	manageKey: string;
	proposal: ProposalRecord;
	refetchDao: () => void;
}

const schema = z.object({});

export function QueueProposalAction({ proposal, manageKey, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();

	const { handleSubmit, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async () => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::obc_system::queue_proposal_action`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(manageKey!),
					tx.object(proposal.proposal_uid),
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

	return (
		<form
			onSubmit={handleSubmit(() => {
				execute.mutateAsync().catch((e) => {
					console.error(`failed to queue proposal action`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
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
