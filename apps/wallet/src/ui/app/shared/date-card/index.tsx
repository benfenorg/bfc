// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import { formatDate } from '_helpers';

type DateCardProps = {
	timestamp: number;
	size: 'sm' | 'md';
};

export function DateCard({ timestamp }: DateCardProps) {
	const txnDate = formatDate(timestamp, ['month', 'day', 'hour', 'minute']);

	return (
		<Text color="bfc-text3" weight="normal" variant="body">
			{txnDate}
		</Text>
	);
}
