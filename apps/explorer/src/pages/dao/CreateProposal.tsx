// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { type ObcDao } from '@mysten/sui.js/src/client';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { hexToBytes } from '@noble/hashes/utils';
import { useMutation } from '@tanstack/react-query';
import BigNumber from 'bignumber.js';
import { z } from 'zod';

import { useZodForm } from '~/hooks/useZodForm';
import { Input } from '~/ui/Input';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	manageKey: string;
	dao: ObcDao;
	refetchDao: () => void;
}

const schema = z.object({
	amount: z
		.string()
		.regex(/\d+/)
		.transform(Number)
		.refine((n) => n >= 200),
	action: z.string().trim(),
});

export function CreateProposal({ manageKey, refetchDao, dao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();

	const { handleSubmit, register } = useZodForm({
		schema: schema,
	});

	const create = useMutation({
		mutationFn: async ({ amount, action }: { amount: number; action: string }) => {
			const bigIntAmount = BigInt(new BigNumber(amount).shiftedBy(9).integerValue().toString());

			const tx = new TransactionBlock();
			const coin = tx.splitCoins(tx.gas, [tx.pure(bigIntAmount)]);

			tx.moveCall({
				target: `0xc8::obc_system::propose`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(manageKey!),
					tx.pure(20),
					coin,
					tx.pure(Number.parseInt(action)),
					tx.pure(6000000),
					tx.object('0x6'),
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
			onSubmit={handleSubmit((formData) => {
				create.mutateAsync(formData).catch((e) => {
					console.error(`failed to create proposal`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Selector
				{...register('action')}
				label="action"
				options={Object.values(dao?.action_record || {}).map((i) => ({
					value: i.action_id.toString(),
					label: new TextDecoder().decode(hexToBytes(i.name.replace(/^0x/, ''))),
				}))}
			/>
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
