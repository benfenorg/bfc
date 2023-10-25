// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Refresh16 } from '@mysten/icons';
import { useCallback, useContext } from 'react';

import { DaoContext } from '~/context';

export function Refresh() {
	const { refetch } = useContext(DaoContext)!;

	const onClick = useCallback(() => {
		refetch();
	}, [refetch]);

	return (
		<Refresh16
			className="fixed bottom-10 right-10 h-12 w-12 cursor-pointer rounded-full border-solid border-bfc-border opacity-60 shadow hover:opacity-70 active:opacity-100"
			onClick={onClick}
		/>
	);
}
