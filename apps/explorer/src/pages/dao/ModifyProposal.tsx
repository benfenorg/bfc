// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
} from '@benfen/bfc.js';
import { ProposalStatus } from '@benfen/bfc.js/client';
import { useZodForm } from '@mysten/core';
import { Button } from '@mysten/ui';
import { useWalletKit } from '@benfen/wallet-kit';
import { useMutation } from '@tanstack/react-query';
import { useContext, useMemo } from 'react';
import { Controller } from 'react-hook-form';
import { z } from 'zod';

import { DaoContext } from '~/context';
import { useDryRunTransactionBlock } from '~/hooks/useDryRunTransactionBlock';
import { Selector } from '~/ui/Selector';
import { ADDRESS } from '~/utils/constants';

const schema = z.object({
	status: z.number({ required_error: 'must select status' }),
});

export function ModifyProposalObj() {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();
	const { proposal, refetch } = useContext(DaoContext)!;
	const dryRun = useDryRunTransactionBlock();

	const { handleSubmit, formState, control } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ status }: { status: number }) => {
			const tx = new TransactionBlock();

			tx.moveCall({
				target: `0xc8::bfc_system::modify_proposal`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.BFC_SYSTEM_STATE),
					tx.object(proposal!.proposal_uid),
					tx.pure(status),
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
			<Controller
				control={control}
				name="status"
				render={({ field: { value, onChange } }) => (
					<Selector label="status" options={options} value={value} onChange={onChange} />
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
