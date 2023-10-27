// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import * as Sentry from '@sentry/browser';
import { getSentryConfig } from '_src/shared/sentry-config';

export function initSentry() {
	Sentry.addTracingExtensions();
	Sentry.init(
		getSentryConfig({
			tracesSampler: () => {
				return false;
			},
		}),
	);
}

export const captureException = Sentry.captureException;
export const captureMessage = Sentry.captureMessage;
