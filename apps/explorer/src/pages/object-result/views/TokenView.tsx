// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin, CoinFormat } from '@mysten/core';
import { type SuiObjectResponse } from '@benfen/bfc.js/client';
import { normalizeSuiAddress, SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { Text } from '@mysten/ui';
import { useState, useEffect } from 'react';

import { trimStdLibPrefix, genFileTypeMsg } from '../../../utils/stringUtils';
import { LinkOrTextDescriptionItem } from '../LinkOrTextDescriptionItem';
import { DynamicFieldsCard } from '~/components/Object/DynamicFieldsCard';
import { ObjectFieldsCard } from '~/components/Object/ObjectFieldsCard';
import TransactionBlocksForAddress from '~/components/TransactionBlocksForAddress/TransactionBlocksForAddress';
import { useResolveVideo } from '~/hooks/useResolveVideo';
import { DescriptionList, DescriptionItem } from '~/ui/DescriptionList';
import { AddressLink, ObjectLink, TransactionLink } from '~/ui/InternalLink';
import { Link } from '~/ui/Link';
import { ObjectDetails } from '~/ui/ObjectDetails';
import { TabHeader } from '~/ui/Tabs';
import { extractName, parseImageURL, parseObjectType } from '~/utils/objectUtils';

export function TokenView({ data }: { data: SuiObjectResponse }) {
	const display = data.data?.display?.data;
	const imgUrl = parseImageURL(display);
	const objOwner = data.data?.owner;
	const name = extractName(display);
	const objectId = data.data?.objectId!;
	const objectType = parseObjectType(data);
	const storageRebate = data.data?.storageRebate;
	const [storageRebateFormatted, symbol] = useFormatCoin(
		storageRebate,
		SUI_TYPE_ARG,
		CoinFormat.FULL,
	);
	const [fileType, setFileType] = useState<undefined | string>(undefined);

	const video = useResolveVideo(data);

	useEffect(() => {
		const controller = new AbortController();
		genFileTypeMsg(imgUrl, controller.signal)
			.then((result) => setFileType(result))
			.catch((err) => console.log(err));

		return () => {
			controller.abort();
		};
	}, [imgUrl]);

	const genhref = (objType: string) => {
		const metadataarr = objType.split('::');
		const address = normalizeSuiAddress(metadataarr[0]);
		return `/object/${address}?module=${metadataarr[1]}`;
	};

	return (
		<div className="flex flex-col flex-nowrap gap-14">
			<TabHeader size="lineMdOne" title="Details" noGap>
				<div className="flex flex-col md:flex-row md:divide-x md:divide-gray-45">
					<div className="flex-1 divide-y divide-gray-45 pb-6 md:basis-2/3 md:pb-0 md:pr-10">
						<div className="py-4 pb-7">
							<DescriptionList>
								{objOwner ? (
									<DescriptionItem title="Owner" data-testid="owner">
										{objOwner === 'Immutable' ? (
											'Immutable'
										) : 'Shared' in objOwner ? (
											'Shared'
										) : 'ObjectOwner' in objOwner ? (
											<ObjectLink objectId={objOwner.ObjectOwner} />
										) : (
											<AddressLink address={objOwner.AddressOwner} />
										)}
									</DescriptionItem>
								) : null}
								<DescriptionItem title="Object ID">
									<ObjectLink objectId={data.data?.objectId!} noTruncate />
								</DescriptionItem>
								<DescriptionItem title="Type">
									{/* TODO: Support module links on `ObjectLink` */}
									<Link to={genhref(objectType)} variant="mono">
										{trimStdLibPrefix(objectType)}
									</Link>
								</DescriptionItem>
								<DescriptionItem title="Version">
									<Text variant="body/medium" color="steel-darker">
										{data.data?.version}
									</Text>
								</DescriptionItem>
								<DescriptionItem title="Last Transaction Block Digest">
									<TransactionLink digest={data.data?.previousTransaction!} noTruncate />
								</DescriptionItem>
							</DescriptionList>
						</div>
						{display ? (
							<div className="py-4 pb-7">
								<DescriptionList>
									<LinkOrTextDescriptionItem title="Name" value={name} />
									<LinkOrTextDescriptionItem title="Description" value={display.description} />
									<LinkOrTextDescriptionItem title="Creator" value={display.creator} parseUrl />
									<LinkOrTextDescriptionItem title="Link" value={display.link} parseUrl />
									<LinkOrTextDescriptionItem title="Website" value={display.project_url} parseUrl />
								</DescriptionList>
							</div>
						) : null}
						{storageRebate && (
							<div className="py-4 pb-7">
								<DescriptionList>
									<DescriptionItem title="Storage Rebate">
										<div className="leading-1 flex items-end gap-0.5">
											<Text variant="body/medium" color="steel-darker">
												{storageRebateFormatted}
											</Text>
											<Text variant="captionSmall/normal" color="steel">
												{symbol}
											</Text>
										</div>
									</DescriptionItem>
								</DescriptionList>
							</div>
						)}
					</div>
					{imgUrl !== '' && (
						<div className="min-w-0 border-0 border-t border-solid border-gray-45 pt-6 md:basis-1/3 md:border-t-0 md:pl-10">
							<div className="flex flex-row flex-nowrap gap-5">
								<ObjectDetails
									image={imgUrl}
									video={video}
									name={name || display?.description || trimStdLibPrefix(objectType)}
									type={video ? 'Video' : fileType ?? ''}
									variant="large"
								/>
							</div>
						</div>
					)}
				</div>
			</TabHeader>
			<ObjectFieldsCard id={objectId} />
			<DynamicFieldsCard id={objectId} />
			<TransactionBlocksForAddress address={objectId} isObject />
		</div>
	);
}
