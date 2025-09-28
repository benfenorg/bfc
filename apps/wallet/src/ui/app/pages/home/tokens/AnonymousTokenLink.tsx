// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { getEphemeralValue } from '_src/background/session-ephemeral-values';
import { getAnonymousRestoreValue } from '_src/shared/anonymous-api';
import { fromExportedKeypair } from '_src/shared/utils/from-exported-keypair';
import { useChainData } from '_src/ui/app/hooks';
import { useActiveAccount } from '_src/ui/app/hooks/useActiveAccount';
import { Text } from '_src/ui/app/shared/text';
import { type StructTag } from '@benfen/bfc.js/bcs/bcs';
import { bfc2HexAddress, parseStructTag } from '@benfen/bfc.js/utils';
import { type AnonymousCoinFields } from '@mysten/core';
import { useMutation } from '@tanstack/react-query';
import { useMemo } from 'react';
import { Link } from 'react-router-dom';

export const AnonymousTokenLink = ({ token }: { token: AnonymousCoinFields }) => {
	const activeAccount = useActiveAccount();
	const { ANONYMOUS_RPC } = useChainData();

	const symbol = useMemo(() => {
		const parsed = parseStructTag(token.balance.type);
		return (parsed.typeParams[0] as unknown as typeof StructTag).name;
	}, [token]);

	const { mutate, data } = useMutation({
		mutationKey: ['get-anonymous-value', token.id.id],
		mutationFn: async () => {
			const ephemeral = (await getEphemeralValue(activeAccount!.id)) as { keyPair: string };
			const keypair = fromExportedKeypair(ephemeral.keyPair);

			const tokenId = bfc2HexAddress(token.id.id);
			const signature = await keypair.sign(new TextEncoder().encode(tokenId));

			const res = await getAnonymousRestoreValue(ANONYMOUS_RPC, {
				value1: token.balance.fields.value1,
				value2: token.balance.fields.value2,
				signature: Array.from(signature),
				objectid: tokenId,
				publickey: Array.from(keypair.getPublicKey().toRawBytes()),
			});
			return res;
		},
	});

	return (
		<Link
			className="flex gap-2.5 w-full py-3 pl-1.5 pr-2 justify-center items-center rounded hover:bg-sui/10 cursor-pointer no-underline relative group"
			to={`/transfer-anonymous`}
		>
			<div className="flex flex-1 gap-1.5 justify-between items-center">
				<Text variant={'body'} color={'steel-darker'} weight={'medium'}>
					{symbol}
				</Text>
				<div className={'flex flex-col items-end'}>
					<Text variant={'body'} color={'steel-darker'} weight={'medium'}>
						{data?.data.result.result1 ?? '***'}
					</Text>
					<Text variant={'body'} color={'steel-darker'} weight={'medium'}>
						{data?.data.result.result2 ?? '***'}
					</Text>
				</div>
			</div>
			<div
				className={
					'absolute hidden group-hover:flex rounded bg-sui/10 p-2 cursor-pointer shadow bottom-[calc(100%)] right-0'
				}
				onClick={(e) => {
					e.stopPropagation();
					e.preventDefault();
					mutate();
				}}
			>
				查看余额
			</div>
		</Link>
	);
};
