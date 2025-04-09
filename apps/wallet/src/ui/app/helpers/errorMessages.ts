// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { LockedDeviceError } from '@ledgerhq/errors';

/**
 * Helper method for producing user-friendly error messages from Signer operations
 * from SignerWithProvider instances (e.g., signTransaction, getAddress, and so forth)
 */
export function getSignerOperationErrorMessage(error: unknown) {
	return (
		getSuiApplicationErrorMessage(error) || (error as Error).message || 'Something went wrong.'
	);
}

/**
 * Helper method for producing user-friendly error messages from errors that arise from
 * operations on the Sui Ledger application
 */
export function getSuiApplicationErrorMessage(error: unknown) {
	if (error instanceof LockedDeviceError) {
		return 'Your device is locked. Unlock it and try again.';
	}
	return null;
}
