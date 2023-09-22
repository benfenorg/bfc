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
import { bytesToHex } from '@noble/hashes/utils';
import { useMutation } from '@tanstack/react-query';
import { z } from 'zod';

import { useZodForm } from '~/hooks/useZodForm';
import { Input } from '~/ui/Input';
import { ADDRESS } from '~/utils/constants';

export interface Props {
	manageKey: string;
	refetchDao: () => void;
}

const schema = z.object({
	text: z.string().trim().min(1),
});

export function CreateDaoAction({ manageKey, refetchDao }: Props) {
	const { isConnected, signAndExecuteTransactionBlock } = useWalletKit();

	const { handleSubmit, register } = useZodForm({
		schema: schema,
	});

	const execute = useMutation({
		mutationFn: async ({ text }: { text: string }) => {
			const tx = new TransactionBlock();
			tx.moveCall({
				target: `0xc8::obc_system::create_obcdao_action`,
				typeArguments: [],
				arguments: [
					tx.object(ADDRESS.OBC_SYSTEM_STATE),
					tx.object(manageKey),
					tx.object(`0x${bytesToHex(new TextEncoder().encode(text))}`),
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
				execute.mutateAsync(formData).catch((e) => {
					console.error(`failed to create dao action`, e);
				});
			})}
			autoComplete="off"
			className="flex flex-col flex-nowrap items-stretch gap-4"
		>
			<Input label="text" {...register('text')} />
			<div className="flex items-stretch gap-1.5">
				<Button variant="primary" type="submit" loading={execute.isLoading} disabled={!isConnected}>
					execute
				</Button>
			</div>
			{execute.error ? <div className="">{(execute.error as Error).message || 'Error'}</div> : null}
			{execute.data && <div>{getTransactionDigest(execute.data)}</div>}
		</form>
	);
}
