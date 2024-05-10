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

// function AddressResultPageHeader({ address, loading }: { address: string; loading?: boolean }) {
// 	const { data: domainName, isFetching } = useResolveSuiNSName(address);

// 	return (
// 		<PageHeader
// 			loading={loading || isFetching}
// 			type="Address"
// 			title={address}
// 			subtitle={domainName}
// 			// before={<Domain32 className="h-6 w-6 text-steel-darker sm:h-10 sm:w-10" />}
// 		/>
// 	);
// }

// function SuiNSAddressResultPageHeader({ name }: { name: string }) {
// 	const { data: address, isFetching } = useResolveSuiNSAddress(name);

// 	return <AddressResultPageHeader address={address ?? name} loading={isFetching} />;
// }

function AddressResult({ address }: { address: string }) {
	const { data: domainName } = useResolveSuiNSName(address);

	return (
		<div className="space-y-12">
			<PageHeader type="Address" title={address} subtitle={domainName} />
			<div>
				<div className="border-b border-bfc-border pb-4 md:mt-12">
					<Heading color="steel-darker" variant="heading6/medium">
						Owned Objects
					</Heading>
				</div>
				<ErrorBoundary>
					<div className="flex flex-col gap-10 border-b border-bfc-border md:flex-row">
						<div className="flex-1  overflow-hidden pb-3">
							<OwnedCoins id={address} />
						</div>
						<div className="hidden w-px bg-bfc-border md:block" />
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
