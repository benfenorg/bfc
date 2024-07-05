// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

const BENFEN_NS_NAME_REGEX =
	/^(?:[a-z0-9][a-z0-9-]{0,62}(?:\.[a-z0-9][a-z0-9-]{0,62})*)?@[a-z0-9][a-z0-9-]{0,62}$/i;
const BENFEN_NS_DOMAIN_REGEX = /^(?:[a-z0-9][a-z0-9-]{0,62}\.)+benfen$/i;
const MAX_BENFEN_NS_NAME_LENGTH = 235;

export function isValidBenfenNSName(name: string): boolean {
	if (name.length > MAX_BENFEN_NS_NAME_LENGTH) {
		return false;
	}

	if (name.includes('@')) {
		return BENFEN_NS_NAME_REGEX.test(name);
	}

	return BENFEN_NS_DOMAIN_REGEX.test(name);
}

export function normalizeBenfenNSName(name: string, format: 'at' | 'dot' = 'at'): string {
	const lowerCase = name.toLowerCase();
	let parts;

	if (lowerCase.includes('@')) {
		if (!BENFEN_NS_NAME_REGEX.test(lowerCase)) {
			throw new Error(`Invalid BenfenNS name ${name}`);
		}
		const [labels, domain] = lowerCase.split('@');
		parts = [...(labels ? labels.split('.') : []), domain];
	} else {
		if (!BENFEN_NS_DOMAIN_REGEX.test(lowerCase)) {
			throw new Error(`Invalid BenfenNS name ${name}`);
		}
		parts = lowerCase.split('.').slice(0, -1);
	}

	if (format === 'dot') {
		return `${parts.join('.')}.benfen`;
	}

	return `${parts.slice(0, -1).join('.')}@${parts[parts.length - 1]}`;
}
