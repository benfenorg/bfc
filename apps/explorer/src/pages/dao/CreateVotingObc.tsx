// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import BigNumber from 'bignumber.js';
import { z } from 'zod';

import { useZodForm } from '~/hooks/useZodForm';
import { Input } from '~/ui/Input';
import { ADDRESS } from '~/utils/constants';

export interface Props {}

const schema = z.object({
	amount: z
		.string()
		.regex(/\d+/)
		.transform(Number)
		.refine((n) => n >= 1),
});

export function CreateVotingObc() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();

	const { handleSubmit, register } = useZodForm({
		schema: schema,
	});

	const create = useMutation({
		mutationFn: async ({ amount }: { amount: number }) => {
			const bigIntAmount = BigInt(new BigNumber(amount).shiftedBy(9).integerValue().toString());

			const tx = new TransactionBlock();
			const coin = tx.splitCoins(tx.gas, [tx.pure(bigIntAmount)]);

			tx.moveCall({
				target: `0xc8::obc_system::create_voting_obc`,
				typeArguments: [],
				arguments: [tx.object(ADDRESS.OBC_SYSTEM_STATE), coin],
			});

			const result = await signAndExecuteTransactionBlock({
				transactionBlock: tx,
			});
			if (getExecutionStatusType(result) === 'failure') {
				throw new Error(getExecutionStatusError(result) || 'Transaction failed');
			}
			return result;
		},
		onSuccess: () => {},
	});
	return (
		<form
			onSubmit={handleSubmit((formData) => {
				create.mutateAsync(formData).catch((e) => {
					console.error(`failed to create voting obc`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Input label="amount" type="number" {...register('amount')} />
			<div className="flex items-stretch gap-1.5">
				<Button variant="primary" type="submit" loading={create.isLoading} disabled={!isConnected}>
					create
				</Button>
			</div>
			{create.error ? <div className="">{(create.error as Error).message || 'Error'}</div> : null}
			{create.data && <div>{getTransactionDigest(create.data)}</div>}
		</form>
	);
}
