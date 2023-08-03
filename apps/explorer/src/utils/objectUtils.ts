// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    getObjectType,
    sui2ObcAddress,
    type ObjectOwner,
} from '@mysten/sui.js';

import { findIPFSvalue } from './stringUtils';

export function parseImageURL(display?: Record<string, string> | null) {
	const url = display?.image_url;
	if (url) {
		if (findIPFSvalue(url)) return url;
		// String representing true http/https URLs are valid:
		try {
			new URL(url);
			return url;
		} catch {
			//do nothing
		}
	}
	return '';
}

export function parseObjectType(data: SuiObjectResponse): string {
	// TODO: define better naming and typing here
	const dataType = getObjectType(data);
	if (dataType === 'package') {
		return 'Move Package';
	}
	return dataType ?? 'unknown';
}

export function getOwnerStr(owner: ObjectOwner | string): string {
    let address: string;
    if (typeof owner === 'object') {
        if ('AddressOwner' in owner) {
            address = owner.AddressOwner;
        } else if ('ObjectOwner' in owner) {
            address = owner.ObjectOwner;
        } else if ('Shared' in owner) {
            address = 'Shared';
        } else {
            address = '';
        }
    } else {
        address = owner;
    }
    return sui2ObcAddress(address);
}

export const checkIsPropertyType = (value: any) => ['number', 'string'].includes(typeof value);

export const extractName = (display?: Record<string, string> | null) => {
	if (!display || !('name' in display)) return undefined;
	const name = display.name;
	if (typeof name === 'string') {
		return name;
	}
	return null;
};

export function getDisplayUrl(url?: string) {
	if (url) {
		try {
			const parsedUrl = new URL(url);
			return {
				href: url,
				display: parsedUrl.hostname,
			};
		} catch (e) {
			// do nothing
		}
	}
	return url || null;
}
