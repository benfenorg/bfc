// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@benfen/bfc.js';
import { humanReadableToBfcDigits, hexToString, strToHex } from '@benfen/bfc.js/utils';
import { useZodForm } from '@mysten/core';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@benfen/bfc.js';
import { useMutation } from '@tanstack/react-query';
import { useContext } from 'react';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { useDryRunTransactionBlock } from '~/hooks/useDryRunTransactionBlock';
import { Input } from '~/ui/Input';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

export function CreateProposal() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { dao, refetch, balance } = useContext(DaoContext)!;
	const dryRun = useDryRunTransactionBlock();

	const schema = z.object({
		amount: z
			.string()
			.transform(Number)
			.refine((n) => n >= 200, 'amount should be greater than or equal to 200')
			.refine(
				(n) => humanReadableToBfcDigits(n) <= BigInt(balance?.totalBalance || ''),
				'insufficient balance',
			),
		version: z
			.string()
			.transform(Number)
			.refine((n) => n >= 24, 'version should be greater than or equal to 24'),
		action: z.number({ required_error: 'must select action' }),
		description: z
			.string({ required_error: 'must input describe' })
			.trim()
			.nonempty()
			.refine((v) => strToHex(v).length <= 1000, 'description must be less than 1000 bytes'),
	});

	const { handleSubmit, formState, register, control } = useZodForm({
		schema: schema,
		defaultValues: {
			version: 24,
		},
	});

	const execute = useMutation({
		mutationFn: async ({
			amount,
			action,
			version,
			description,
		}: {
			amount: number;
			action: number;
			version: number;
			description: string;
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
					tx.object(strToHex(description)),
					tx.object(ADDRESS.CLOCK),
				],
			});

			await dryRun(tx);
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
							label: i.action_id.toString() + '-' + hexToString(i.name),
						}))}
						value={value}
						onChange={onChange}
					/>
				)}
			/>

			<Input label="amount" type="number" step="any" {...register('amount')} />
			<Input label="version" type="number" {...register('version')} />
			<Input label="description" {...register('description')} />
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
