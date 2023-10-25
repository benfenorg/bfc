// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useZodForm } from '@mysten/core';
import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useContext } from 'react';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({});

export function JudgeProposalState() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { refetch } = useContext(DaoContext)!;

	const { handleSubmit, formState } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async () => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::judge_proposal_state`,
				typeArguments: [],
				arguments: [tx.object(ADDRESS.BFC_SYSTEM_STATE), tx.pure(Date.now())],
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
			refetch();
		},
	});

	return (
		<form
			onSubmit={handleSubmit(() => {
				execute.mutateAsync().catch((e) => {
					console.error(`failed to judge proposal state`, e);
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
