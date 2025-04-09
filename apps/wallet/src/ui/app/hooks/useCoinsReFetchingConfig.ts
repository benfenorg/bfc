// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

export function useCoinsReFetchingConfig() {
	return {
		refetchInterval: 10_000,
		staleTime: 5_000,
	};
}
