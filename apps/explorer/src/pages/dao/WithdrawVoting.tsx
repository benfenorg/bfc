// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useZodForm } from '@mysten/core';
import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { obcDigitsToHumanReadable } from '@mysten/sui.js/utils';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useMemo } from 'react';
import { z } from 'zod';

import { useGetOBCDaoVotingObc } from '~/hooks/useGetOBCDaoVotingObc';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	refetchDao: () => void;
}

const schema = z.object({
	vote: z.string().trim().nonempty(),
});

export function WithdrawVoting({ refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();

	const { handleSubmit, register, formState } = useZodForm({
		schema: schema,
	});

	const { data: votes = [], refetch: refetchVoting } = useGetOBCDaoVotingObc(
		currentAccount?.address || '',
	);

	const execute = useMutation({
		mutationFn: async ({ vote }: { vote: string }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::withdraw_voting`,
				typeArguments: [],
				arguments: [tx.object(ADDRESS.BFC_SYSTEM_STATE), tx.object(vote)],
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

	const options = useMemo(
		() =>
			votes.map((i) => ({
				label: obcDigitsToHumanReadable(i.principal),
				value: i.id.id,
			})),
		[votes],
	);

	return (
		<form
			onSubmit={handleSubmit((formData) => {
				execute.mutateAsync(formData).catch((e) => {
					console.error(`failed to withdraw voting bfc`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Selector label="voting" options={options} {...register('vote')} />
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
