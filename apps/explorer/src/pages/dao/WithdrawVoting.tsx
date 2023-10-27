// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useZodForm } from '@mysten/core';
import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { bfcDigitsToHumanReadable } from '@mysten/sui.js/utils';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useContext, useMemo } from 'react';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({
	vote: z.string({ required_error: 'must select voting' }).trim().nonempty(),
});

export function WithdrawVoting() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { votingBfcs, refetch } = useContext(DaoContext)!;

	const { handleSubmit, formState, control } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ vote }: { vote: string }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::withdraw_voting`,
				typeArguments: [],
				arguments: [tx.object(ADDRESS.BFC_SYSTEM_STATE), tx.object(vote), tx.object(ADDRESS.CLOCK)],
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

	const options = useMemo(
		() =>
			votingBfcs.map((i) => ({
				label: bfcDigitsToHumanReadable(i.principal),
				value: i.id.id,
			})),
		[votingBfcs],
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
