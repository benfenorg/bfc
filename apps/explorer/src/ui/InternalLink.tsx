// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isSuiNSName } from '@mysten/core';
import { sui2BfcAddress } from '@mysten/sui.js';
import { formatAddress, formatDigest } from '@mysten/sui.js/utils';
import { type ReactNode } from 'react';

import { Link, type LinkProps } from '~/ui/Link';

interface BaseInternalLinkProps extends LinkProps {
	noTruncate?: boolean;
	label?: string | ReactNode;
	queryStrings?: Record<string, string>;
}

function createInternalLink<T extends string>(
	base: string,
	propName: T,
	formatter: (id: string) => string = (id) => id,
) {
	return ({
		[propName]: id,
		noTruncate,
		label,
		queryStrings = {},
		...props
	}: BaseInternalLinkProps & Record<T, string>) => {
		const queryString = new URLSearchParams(queryStrings).toString();
		const queryStringPrefix = queryString ? `?${queryString}` : '';

		let converted: string = id;
		if (['address', 'object', 'validator'].includes(base)) {
			const queryIndex = id.indexOf('?');
			if (queryIndex === -1) {
				converted = sui2BfcAddress(id);
			} else {
				converted = sui2BfcAddress(id.slice(0, queryIndex)) + id.slice(queryIndex);
			}
		}
		const truncatedAddress = noTruncate ? converted : formatter(converted);
		return (
			<Link variant="mono" to={`/${base}/${encodeURI(converted)}${queryStringPrefix}`} {...props}>
				{label || truncatedAddress}
			</Link>
		);
	};
}

export const EpochLink = createInternalLink('epoch', 'epoch');
export const CheckpointLink = createInternalLink('checkpoint', 'digest', formatAddress);
export const CheckpointSequenceLink = createInternalLink('checkpoint', 'sequence');
export const AddressLink = createInternalLink('address', 'address', (addressOrNs) => {
	if (isSuiNSName(addressOrNs)) {
		return addressOrNs;
	}
	return formatAddress(addressOrNs);
});
export const ObjectLink = createInternalLink('object', 'objectId', formatAddress);
export const TransactionLink = createInternalLink('txblock', 'digest', formatDigest);
export const ValidatorLink = createInternalLink('validator', 'address', formatAddress);
