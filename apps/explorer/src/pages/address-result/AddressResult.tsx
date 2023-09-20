// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isSuiNSName, useResolveSuiNSAddress, useResolveSuiNSName } from '@mysten/core';
import { Heading, LoadingIndicator } from '@mysten/ui';
import { useParams } from 'react-router-dom';

import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { TransactionsForAddress } from '../../components/transactions/TransactionsForAddress';
import { PageLayout } from '~/components/Layout/PageLayout';
import { OwnedCoins } from '~/components/OwnedCoins';
import { OwnedObjects } from '~/components/OwnedObjects';
import { PageHeader } from '~/ui/PageHeader';

function AddressResult({ address }: { address: string }) {
	const { data: domainName } = useResolveSuiNSName(address);

	return (
		<div className="space-y-12">
			<PageHeader type="Address" title={address} subtitle={domainName} />
			<div>
				<div className="border-b border-obc-border pb-4 md:mt-12">
					<Heading color="steel-darker" variant="heading6/medium">
						Owned Objects
					</Heading>
				</div>
				<ErrorBoundary>
					<div className="flex flex-col gap-10 border-b border-obc-border md:flex-row">
						<div className="flex-1  overflow-hidden pb-3">
							<OwnedCoins id={address} />
						</div>
						<div className="hidden w-px bg-obc-border md:block" />
						<div className="flex-1  overflow-hidden pb-3">
							<OwnedObjects id={address} />
						</div>
					</div>
				</ErrorBoundary>
			</div>

			<div>
				<ErrorBoundary>
					<div className="mt-2">
						<TransactionsForAddress address={address} type="address" />
					</div>
				</ErrorBoundary>
			</div>
		</div>
	);
}

function SuiNSAddressResult({ name }: { name: string }) {
	const { isFetched, data } = useResolveSuiNSAddress(name);

	if (!isFetched) {
		return <LoadingIndicator />;
	}

	// Fall back into just trying to load the name as an address anyway:
	return <AddressResult address={data ?? name} />;
}

export default function AddressResultPage() {
	const { id } = useParams();
	return (
		<PageLayout
			content={
				isSuiNSName(id!) ? <SuiNSAddressResult name={id!} /> : <AddressResult address={id!} />
			}
		/>
	);
}
