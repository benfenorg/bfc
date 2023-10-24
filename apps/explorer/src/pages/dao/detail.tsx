// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ProposalStatus, type ProposalRecordWithStatus } from '@mysten/sui.js/client';
import { Heading } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';
import { hexToBytes } from '@noble/hashes/utils';
import dayjs from 'dayjs';
import { useMemo, createContext, useContext } from 'react';
import { useParams } from 'react-router-dom';

import { CastVote } from './CastVote';
import { ChangeVote } from './ChangeVote';
import { JudgeProposalState } from './JudgeProposalState';
import { ModifyProposalObj } from './ModifyProposal';
import { QueueProposalAction } from './QueueProposalAction';
import { RevokeVote } from './RevokeVote';
import { UnvoteVotes } from './UnvoteVotes';
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan, RejectSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useGetBFCDaoManageKey } from '~/hooks/useGetBFCDaoManageKey';
import { useGetDao } from '~/hooks/useGetDao';
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
			<div className="rounded-md border border-bfc-border px-3 py-6">
				<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">
					Details
				</div>
				<div className="mt-5 space-y-3">
					<div className="flex justify-between">
						<div className="text-pBody text-bfc-text2">ID</div>
						<div className="text-pBody text-bfc-text1">{proposal.pid}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-bfc-text2">Status</div>
						<div className="text-pBody text-bfc-text1">{ProposalStatus[proposal.status]}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-bfc-text2">Proposer</div>
						<div className="text-pBody text-bfc-text1">{proposal.proposer}</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-bfc-text2">End Time</div>
						<div className="text-pBody text-bfc-text1">
							{dayjs(proposal.end_time).format('YYYY-MM-DD HH:mm:ss')}
						</div>
					</div>
					<div className="flex justify-between">
						<div className="text-pBody text-bfc-text2">Message Board</div>
						<div className="text-pBody text-bfc-text1">12345</div>
					</div>
				</div>
			</div>
			<div className="mt-5 rounded-md border border-bfc-border px-3 py-6">
				<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">
					Description
				</div>
				<div className="mt-5 text-pBody text-bfc-text1">
					Galaxy is a digital asset and blockchain leader helping institutions, startups, and
					qualified individuals shape a changing economy. We provide platform solutions custom-made
					for a digitally native ecosystem.
				</div>
			</div>
		</div>
	);
}

function PoolDetail() {
	const { proposal, refetch, manageKey } = useContext(DaoDetailContext)!;
	const total = proposal.for_votes + proposal.against_votes;

	return (
		<div>
			<div className="flex justify-between">
				<div className="flex items-baseline gap-1">
					<div className="text-heading4 font-semibold">
						{total === 0 ? 0 : ((proposal.against_votes / total) * 100).toFixed(1) + '%'}
					</div>
					<div className="text-body text-bfc-text2">Opposition</div>
				</div>
				<div className="flex items-baseline gap-1">
					<div className="text-heading4 font-semibold">
						{total === 0 ? 0 : ((proposal.for_votes / total) * 100).toFixed(1) + '%'}
					</div>
					<div className="text-body text-bfc-text2">Agree</div>
				</div>
			</div>
			<div className="relative my-3 h-1 overflow-hidden rounded-br-lg rounded-tl-lg bg-bfc-green">
				<div
					className="absolute h-1 bg-bfc-red"
					style={{
						width: total === 0 ? '50%' : (proposal.against_votes / total) * 100 + '%',
					}}
				/>
			</div>

			<div className="flex h-4.5 items-center gap-2">
				<div>
					<span className="text-body text-bfc-text2">Voted</span>
					<span className="text-body font-medium text-bfc-text1">&nbsp;{total}</span>
				</div>
				<div className="h-3 w-[1px] bg-bfc-border" />
				<div>
					<span className="text-body text-bfc-text2">Quorum</span>
					<span className="text-body font-medium text-bfc-text1">
						&nbsp;{proposal.quorum_votes}
					</span>
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
					<>
						<DisclosureBox title="cast vote" defaultOpen={false}>
							<CastVote proposal={proposal} refetchDao={refetch} />
						</DisclosureBox>
						<DisclosureBox title="change vote" defaultOpen={false}>
							<ChangeVote proposal={proposal} refetchDao={refetch} />
						</DisclosureBox>
						<DisclosureBox title="revoke vote" defaultOpen={false}>
							<RevokeVote proposal={proposal} refetchDao={refetch} />
						</DisclosureBox>
					</>
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
function Pool() {
	return (
		<div className="h-max rounded-md border border-bfc-border px-3 py-6">
			<div className="text-heading6 font-semibold text-steel-darker md:text-heading4">
				Voting Pool
			</div>
			<div className="mt-5">
				<PoolDetail />
			</div>
		</div>
	);
}

function DaoContent() {
	const { id } = useParams<{ id: string }>();
	const { currentAccount } = useWalletKit();
	const { data: daoData, isLoading, refetch } = useGetDao();
	const { data: manageKey } = useGetBFCDaoManageKey(currentAccount?.address || '');

	const data = useMemo(() => {
		if (!daoData) {
			return undefined;
		}
		const proposal = daoData.proposal_record.find((i) => i.proposal_uid === id)!;
		return {
			...proposal,
			status: daoData.current_proposal_status[proposal!.pid]?.status || ProposalStatus.Pending,
		};
	}, [daoData, id]);

	if (isLoading || !data) {
		return null;
	}

	return (
		<DaoDetailContext.Provider value={{ proposal: data, refetch, manageKey }}>
			<div>
				<div className="flex flex-col gap-2 rounded-md border-l-4 border-bfc-border bg-bfc-card p-5 lg:flex-row">
					<div className="flex min-w-0 flex-col gap-1">
						<div className="flex gap-1">
							{data.status === ProposalStatus.Agree ? (
								<AgreeSpan />
							) : data.status === ProposalStatus.Defeat ? (
								<RejectSpan />
							) : (
								<StatusSpan text={ProposalStatus[data.status]} />
							)}
						</div>
						<div className="min-w-0 break-words">
							<Heading as="h2" variant="heading3/semibold" color="bfc-text1" mono>
								{new TextDecoder().decode(
									hexToBytes(daoData!.action_record[data.action.action_id].name.replace(/^0x/, '')),
								)}
							</Heading>
						</div>
					</div>
				</div>
				<div className="mt-6 grid grid-cols-2 gap-5">
					<DaoContentDetail />
					<Pool />
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
