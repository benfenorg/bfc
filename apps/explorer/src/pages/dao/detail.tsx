// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ProposalStatus, type ProposalRecordWithStatus } from '@mysten/sui.js/client';
import { Heading } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { useMemo, createContext, useContext } from 'react';
import { useParams } from 'react-router-dom';

import { CastVote } from './CastVote';
import { JudgeProposalState } from './JudgeProposalState';
import { ModifyProposalObj } from './ModifyProposal';
import { QueueProposalAction } from './QueueProposalAction';
import { UnvoteVotes } from './UnvoteVotes';
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useGetDao } from '~/hooks/useGetDao';
import { useGetOBCDaoManageKey } from '~/hooks/useGetOBCDaoManageKey';
import { DisclosureBox } from '~/ui/DisclosureBox';
import { PageHeader } from '~/ui/PageHeader';

export interface DaoDetailContextProps {
	proposal: ProposalRecordWithStatus;
	manageKey?: string;
	refetch: () => void;
}

const DaoDetailContext = createContext<DaoDetailContextProps | undefined>(undefined);

function DaoContentDetail() {
	const { proposal } = useContext(DaoDetailContext)!;

	return (
		<div>
			<div className="rounded-md border border-obc-border px-3 py-6">
				<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">详情</div>
				<div className="mt-5 space-y-3">
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">ID</div>
						<div className="text-pBody text-obc-text1">{proposal.pid}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">状态</div>
						<div className="text-pBody text-obc-text1">12345</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">创建者</div>
						<div className="text-pBody text-obc-text1">{proposal.proposer}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-obc-text2">结束时间</div>
						<div className="text-pBody text-obc-text1">{proposal.end_time}</div>
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
	const { proposal, refetch, manageKey } = useContext(DaoDetailContext)!;

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
			<div className="my-3 flex flex-col gap-2">
				<DisclosureBox title="modify proposal" defaultOpen={false}>
					<ModifyProposalObj proposal={proposal} refetchDao={refetch} />
				</DisclosureBox>
				<DisclosureBox title="judge proposal state" defaultOpen={false}>
					<JudgeProposalState refetchDao={refetch} />
				</DisclosureBox>
				{proposal.status === ProposalStatus.Active && (
					<DisclosureBox title="cast vote" defaultOpen={false}>
						<CastVote proposal={proposal} refetchDao={refetch} />
					</DisclosureBox>
				)}
				{proposal.end_time < Date.now() && (
					<DisclosureBox title="unvote votes" defaultOpen={false}>
						<UnvoteVotes proposal={proposal} refetchDao={refetch} />
					</DisclosureBox>
				)}
				{manageKey && proposal.status === ProposalStatus.Agree && (
					<DisclosureBox title="queue proposal action">
						<QueueProposalAction proposal={proposal} refetchDao={refetch} manageKey={manageKey} />
					</DisclosureBox>
				)}
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
	const { id } = useParams<{ id: string }>();
	const { currentAccount } = useWalletKit();
	const { data: daoData, isLoading, refetch } = useGetDao();
	const { data: manageKey } = useGetOBCDaoManageKey(currentAccount?.address || '');

	const data = useMemo(() => {
		if (!daoData) {
			return undefined;
		}
		const proposal = daoData.proposal_record.find((i) => i.proposal_uid === id)!;
		return {
			...proposal,
			status: daoData.current_proposal_status[proposal!.pid].status,
		};
	}, [daoData, id]);

	if (isLoading || !data) {
		return null;
	}

	return (
		<DaoDetailContext.Provider value={{ proposal: data, refetch, manageKey }}>
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
					<DaoContentDetail />
					<Poll />
				</div>
			</div>
		</DaoDetailContext.Provider>
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
