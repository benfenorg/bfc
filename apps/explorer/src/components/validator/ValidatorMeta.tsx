// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ArrowUpRight12 } from '@mysten/icons';
import { sui2ObcAddress, formatAddress } from '@mysten/sui.js';
import { type SuiValidatorSummary } from '@mysten/sui.js/client';
import { Heading, Text } from '@mysten/ui';

import { StakeButton } from './StakeButton';
import { CopyToClipboard } from '~/ui/CopyToClipboard';
import { DescriptionList, DescriptionItem } from '~/ui/DescriptionList';
import { Divider } from '~/ui/Divider';
import { ImageIcon } from '~/ui/ImageIcon';
import { AddressLink } from '~/ui/InternalLink';

type ValidatorMetaProps = {
	validatorData: SuiValidatorSummary;
};

export function ValidatorMeta({ validatorData }: ValidatorMetaProps) {
	const validatorPublicKey = validatorData.protocolPubkeyBytes;
	const validatorName = validatorData.name;
	const logo = validatorData.imageUrl;
	const description = validatorData.description;
	const projectUrl = validatorData.projectUrl;

	return (
		<>
			<div className="flex gap-3">
				<ImageIcon src={logo} label={validatorName} fallback={validatorName} size="xl" />
				<div className="flex flex-1 flex-col justify-center">
					<Heading as="h1" variant="heading3/bold" color="steel-darker">
						{validatorName}
					</Heading>
					{projectUrl && (
						<a
							href={projectUrl}
							target="_blank"
							rel="noreferrer noopener"
							className="mt-2.5 inline-flex items-center gap-1.5 text-body font-medium text-sui-dark no-underline"
						>
							{projectUrl.replace(/\/$/, '')}
							<ArrowUpRight12 className="text-steel" />
						</a>
					)}
				</div>
				<div className="flex items-center">
					<div>
						<StakeButton />
					</div>
				</div>
			</div>
			<div className="mb-1 mt-5">
				<Divider />
			</div>

			<div className="min-w-0 basis-full break-words md:basis-2/3">
				<DescriptionList>
					<DescriptionItem title="Description" align="start" direction="cloumn">
						<Text variant="pBody/medium" color="gray-90">
							{description || '--'}
						</Text>
					</DescriptionItem>
					<div className="flex gap-3">
						<DescriptionItem title="Pool ID" align="start" direction="cloumn">
							<div className="flex items-start gap-1 break-all">
								<Text variant="pBody/medium" color="steel-darker">
									{formatAddress(sui2ObcAddress(validatorData.stakingPoolId))}
								</Text>
								<CopyToClipboard
									size="md"
									color="steel"
									copyText={sui2ObcAddress(validatorData.stakingPoolId)}
								/>
							</div>
						</DescriptionItem>
						<DescriptionItem title="Address" align="start" direction="cloumn">
							<div className="flex items-start gap-1">
								<AddressLink address={validatorData.suiAddress} />
								<CopyToClipboard
									size="md"
									color="steel"
									copyText={sui2ObcAddress(validatorData.suiAddress)}
								/>
							</div>
						</DescriptionItem>
					</div>
					<DescriptionItem title="Public Key" align="start" direction="cloumn">
						<Text variant="pBody/medium" color="steel-darker">
							{validatorPublicKey}
						</Text>
					</DescriptionItem>
					<DescriptionItem title="Location" align="start" direction="cloumn">
						<Text variant="pBody/medium" color="gray-90">
							--
						</Text>
					</DescriptionItem>
				</DescriptionList>
			</div>
		</>
	);
}
