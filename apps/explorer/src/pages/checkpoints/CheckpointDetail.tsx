// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRpcClient } from '@mysten/core';
import { Text, LoadingIndicator } from '@mysten/ui';
import { useQuery } from '@tanstack/react-query';
import { useParams } from 'react-router-dom';

import { CheckpointTransactionBlocks } from './CheckpointTransactionBlocks';
import { PageLayout } from '~/components/Layout/PageLayout';
import { SuiAmount } from '~/components/Table/SuiAmount';
import { Banner } from '~/ui/Banner';
import { DescriptionFlexList, DescriptionItem } from '~/ui/DescriptionList';
import { EpochLink } from '~/ui/InternalLink';
import { PageHeader } from '~/ui/PageHeader';
import { TabHeader, Tabs, TabsContent, TabsList, TabsTrigger } from '~/ui/Tabs';

export default function CheckpointDetail() {
	const { id } = useParams<{ id: string }>();
	const digestOrSequenceNumber = /^\d+$/.test(id!) ? parseInt(id!, 10) : id;

	const rpc = useRpcClient();
	const { data, isError, isLoading } = useQuery({
		queryKey: ['checkpoints', digestOrSequenceNumber],
		queryFn: () => rpc.getCheckpoint({ id: String(digestOrSequenceNumber!) }),
	});
	return (
		<PageLayout
			content={
				isError ? (
					<Banner variant="error" fullWidth>
						There was an issue retrieving data for checkpoint: {id}
					</Banner>
				) : isLoading ? (
					<LoadingIndicator />
				) : (
					<div className="flex flex-col space-y-12">
						<PageHeader title={data.digest} type="Checkpoint" />
						<div className="space-y-8">
							<Tabs size="lineMd" defaultValue="details">
								<TabsList>
									<TabsTrigger value="details">Details</TabsTrigger>
									<TabsTrigger value="signatures">Signatures</TabsTrigger>
								</TabsList>
								<TabsContent value="details">
									<DescriptionFlexList>
										<DescriptionItem title="Checkpoint Sequence No." direction='cloumn' align="start">
											<Text variant="pBody/medium" color="steel-darker">
												{data.sequenceNumber}
											</Text>
										</DescriptionItem>
										<DescriptionItem direction="cloumn"  align="start" title="Epoch">
											<EpochLink epoch={data.epoch} />
										</DescriptionItem>
										<DescriptionItem direction="cloumn"  align="start" title="Checkpoint Timestamp">
											<Text variant="pBody/medium" color="steel-darker">
												{data.timestampMs
													? new Date(Number(data.timestampMs)).toLocaleString(undefined, {
															month: 'short',
															day: 'numeric',
															year: 'numeric',
															hour: 'numeric',
															minute: '2-digit',
															second: '2-digit',
															hour12: false,
															timeZone: 'UTC',
															timeZoneName: 'short',
													  })
													: '--'}
											</Text>
										</DescriptionItem>
									</DescriptionFlexList>
								</TabsContent>
								<TabsContent value="signatures">
									<Tabs size="lineMdOne" defaultValue="aggregated">
										<TabsList>
											<TabsTrigger value="aggregated">Aggregated Validator Signature</TabsTrigger>
										</TabsList>
										<TabsContent value="aggregated">
											<DescriptionFlexList>
												<DescriptionItem direction="cloumn" align="start" key={data.validatorSignature} title="Signature">
													<Text variant="pBody/medium" color="steel-darker">
														{data.validatorSignature}
													</Text>
												</DescriptionItem>
											</DescriptionFlexList>
										</TabsContent>
									</Tabs>
								</TabsContent>
							</Tabs>

							<TabHeader size="lineMdOne" title="Gas & Storage Fees">
								<DescriptionFlexList>
									<DescriptionItem direction="cloumn" align="start" title="Computation Fee">
										<SuiAmount full amount={data.epochRollingGasCostSummary.computationCost} />
									</DescriptionItem>
									<DescriptionItem direction="cloumn" align="start" title="Storage Fee">
										<SuiAmount full amount={data.epochRollingGasCostSummary.storageCost} />
									</DescriptionItem>
									<DescriptionItem direction="cloumn" align="start" title="Storage Rebate">
										<SuiAmount full amount={data.epochRollingGasCostSummary.storageRebate} />
									</DescriptionItem>
								</DescriptionFlexList>
							</TabHeader>

							<TabHeader size="lineMdOne" title="Checkpoint Transaction Blocks">
								<div className="mt-4">
									<CheckpointTransactionBlocks id={data.sequenceNumber} />
								</div>
							</TabHeader>
						</div>
					</div>
				)
			}
		/>
	);
}
