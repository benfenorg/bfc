// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_src/ui/app/shared/text';
import { type StructTag } from '@benfen/bfc.js/bcs/bcs';
import { parseStructTag } from '@benfen/bfc.js/utils';
import { formatAmount, type AnonymousCoinFields } from '@mysten/core';
import { BigNumber } from 'bignumber.js';
import { useMemo } from 'react';
import { Link } from 'react-router-dom';

export const AnonymousTokenLink = ({ token }: { token: AnonymousCoinFields }) => {
	const symbol = useMemo(() => {
		const parsed = parseStructTag(token.balance.type);
		return (parsed.typeParams[0] as unknown as typeof StructTag).name;
	}, [token]);

	const encodeBalance = useMemo(() => {
		const [value1, value2] = token.balance.fields.encode_data.split(',');
		const bn1 = new BigNumber(value1).shiftedBy(-1 * 9);
		const bn2 = new BigNumber(value2).shiftedBy(-1 * 9);
		return `${formatAmount(bn1)}_${formatAmount(bn2)}`;
	}, [token]);

	const balance = useMemo(() => {
		return formatAmount(new BigNumber(token.balance.fields.value).shiftedBy(-1 * 9));
	}, [token]);

	return (
		<Link
			className="flex gap-2.5 w-full py-3 pl-1.5 pr-2 justify-center items-center rounded hover:bg-sui/10 cursor-pointer no-underline"
			to={`/transfer-anonymous`}
		>
			<div className="flex flex-1 gap-1.5 justify-between items-center">
				<Text variant={'body'} color={'steel-darker'} weight={'medium'}>
					{symbol}
				</Text>
				<div className={'flex flex-col items-end'}>
					<Text variant={'body'} color={'steel-darker'} weight={'medium'}>
						{balance}
					</Text>
					<Text variant={'body'} color={'steel-darker'} weight={'medium'}>
						{encodeBalance}
					</Text>
				</div>
			</div>
		</Link>
	);
};
