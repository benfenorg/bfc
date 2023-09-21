// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Heading } from '@mysten/ui';
import { useMemo } from 'react';
import { useParams } from 'react-router-dom';

import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useGetDao } from '~/hooks/useGetDao';
import { PageHeader } from '~/ui/PageHeader';

function DaoContentDeatil() {
	const { id } = useParams<{ id: string }>();
	const { data: daoData, isLoading } = useGetDao();

	const data = useMemo(
		() => daoData?.proposal_record.find((i) => i.pid.toString() === id),
		[daoData, id],
	);

	if (isLoading || !data) {
		return null;
	}

	return (
		<div>
			<div className="rounded-md border border-obc-border px-3 py-6">
				<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">详情</div>
				<div className="mt-5 space-y-3">
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">ID</div>
						<div className="text-pBody text-obc-text1">{data.pid}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">状态</div>
						<div className="text-pBody text-obc-text1">12345</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">创建者</div>
						<div className="text-pBody text-obc-text1">{data.proposer}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">结束时间</div>
						<div className="text-pBody text-obc-text1">{data.end_time}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">留言板</div>
						<div className="text-pBody text-obc-text1">12345</div>
					</div>
				</div>
			</div>
			<div className="mt-5 rounded-md border border-obc-border px-3 py-6">
				<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">
					Description
				</div>
				<div className="mt-5 text-pBody text-obc-text1">
					Galaxy is a digital asset and blockchain leader helping institutions, startups, and
					qualified individuals shape a changing economy. We provide platform solutions custom-made
					for a digitally native ecosystem.
				</div>
			</div>
		</div>
	);
}

function PollDetail() {
	return (
		<div>
			<div className="flex justify-between">
				<div className="flex items-baseline gap-1">
					<div className="text-heading4 font-semibold">50.5%</div>
					<div className="text-body text-obc-text2">同意</div>
				</div>
				<div className="flex items-baseline gap-1">
					<div className="text-heading4 font-semibold">50.5%</div>
					<div className="text-body text-obc-text2">同意</div>
				</div>
			</div>
			<div className="relative my-3 h-1 overflow-hidden rounded-br-lg rounded-tl-lg bg-obc-green">
				<div className="absolute h-1 w-[50%] bg-obc-red" />
			</div>

			<div className="flex h-4.5 items-center gap-2">
				<div>
					<span className="text-body text-obc-text2">已投票</span>
					<span className="text-body font-medium text-obc-text1"> 55.5%</span>
				</div>
				<div className="h-3 w-[1px] bg-obc-border" />
				<div>
					<span className="text-body text-obc-text2">法定门槛</span>
					<span className="text-body font-medium text-obc-text1"> 51%</span>
				</div>
			</div>
		</div>
	);
}
function Poll() {
	return (
		<div className="h-max rounded-md border border-obc-border px-3 py-6">
			<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">投票</div>
			<div className="mt-5">
				<PollDetail />
			</div>
		</div>
	);
}

function DaoContent() {
	return (
		<div>
			<div className="flex flex-col gap-2 rounded-md border-l-4 border-obc-border bg-obc-card p-5 lg:flex-row">
				<div className="flex min-w-0 flex-col gap-1">
					<div className="flex gap-1">
						<AgreeSpan />
						<StatusSpan />
					</div>
					<div className="min-w-0 break-words">
						<Heading as="h2" variant="heading3/semibold" color="obc-text1" mono>
							asdasdasdsa
						</Heading>
					</div>
				</div>
			</div>
			<div className="mt-6 grid grid-cols-2 gap-5">
				<DaoContentDeatil />
				<Poll />
			</div>
		</div>
	);
}

function DaoDetail() {
	return (
		<PageLayout
			content={
				<div className="mb-10">
					<PageHeader type="Dao" />
					<div>
						<ErrorBoundary>
							<DaoContent />
						</ErrorBoundary>
					</div>
				</div>
			}
		/>
	);
}

export default DaoDetail;
