// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useZodForm } from '@mysten/core';
import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@mysten/sui.js';
import { humanReadableToBfcDigits } from '@mysten/sui.js/utils';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils';
import { useMutation } from '@tanstack/react-query';
import { useContext } from 'react';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { Input } from '~/ui/Input';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({
	amount: z
		.string()
		.transform(Number)
		.refine((n) => n >= 200, 'amount should be greater than or equal to 200'),
	version: z
		.string()
		.transform(Number)
		.refine((n) => n >= 24, 'version should be greater than or equal to 24'),
	action: z.number({ required_error: 'must select action' }),
	describe: z.string({ required_error: 'must input describe' }).trim().nonempty(),
});

export function CreateProposal() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { dao, refetch } = useContext(DaoContext)!;

	const { handleSubmit, formState, register, control } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({
			amount,
			action,
			version,
			describe,
		}: {
			amount: number;
			action: number;
			version: number;
			describe: string;
		}) => {
			const bigIntAmount = humanReadableToBfcDigits(amount);

			const tx = new TransactionBlock();
			const coin = tx.splitCoins(tx.gas, [tx.pure(bigIntAmount)]);

			tx.moveCall({
				target: `0xc8::bfc_system::propose`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.BFC_SYSTEM_STATE),
					tx.pure(version),
					coin,
					tx.pure(action),
					tx.pure(6000000),
					tx.object(`0x${bytesToHex(new TextEncoder().encode(describe))}`),
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
			refetch();
		},
	});
	return (
		<form
			onSubmit={handleSubmit((formData) => {
				execute.mutateAsync(formData).catch((e) => {
					console.error(`failed to create proposal`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Controller
				control={control}
				name="action"
				render={({ field: { value, onChange } }) => (
					<Selector
						label="action"
						options={Object.values(dao?.action_record || {}).map((i) => ({
							value: i.action_id,
							label:
								i.action_id.toString() +
								'-' +
								new TextDecoder().decode(hexToBytes(i.name.replace(/^0x/, ''))),
						}))}
						value={value}
						onChange={onChange}
					/>
				)}
			/>

			<Input label="amount" type="number" step="any" {...register('amount')} />
			<Input label="version" type="number" {...register('version')} />
			<Input label="describe" {...register('describe')} />
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
