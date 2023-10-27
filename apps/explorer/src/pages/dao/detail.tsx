// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ProposalStatus } from '@mysten/sui.js/client';
import { bfcDigitsToHumanReadable, hexToString } from '@mysten/sui.js/utils';
import { Heading, Button } from '@mysten/ui';
import dayjs from 'dayjs';
import { useContext } from 'react';
import { useParams } from 'react-router-dom';

import { CastVote } from './CastVote';
import { ChangeVote } from './ChangeVote';
import { JudgeProposalState } from './JudgeProposalState';
import { ModifyProposalObj } from './ModifyProposal';
import { QueueProposalAction } from './QueueProposalAction';
import { Refresh } from './Refresh';
import { RevokeVote } from './RevokeVote';
import { UnvoteVotes } from './UnvoteVotes';
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan, RejectSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useDaoContext, DaoContext } from '~/context';
import { DisclosureBox } from '~/ui/DisclosureBox';
import { Link } from '~/ui/Link';
import { PageHeader } from '~/ui/PageHeader';

function DaoContentDetail() {
	const { proposal } = useContext(DaoContext)!;

	if (!proposal) {
		return null;
	}

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
						<div className="text-pBody text-bfc-text2">Version</div>
						<div className="text-pBody text-bfc-text1">{proposal.version_id}</div>
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
						<div className="text-pBody text-bfc-text2">Start Time</div>
						<div className="text-pBody text-bfc-text1">
							{dayjs(proposal.start_time).format('YYYY-MM-DD HH:mm:ss')}
						</div>
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
				<div className="mt-5 text-pBody text-bfc-text1">{hexToString(proposal.description)}</div>
			</div>
		</div>
	);
}

function PoolDetail() {
	const { proposal, manageKey, votingBfcs, votes } = useContext(DaoContext)!;
	if (!proposal) {
		return null;
	}

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
					<span className="text-body font-medium text-bfc-text1">
						&nbsp;{bfcDigitsToHumanReadable(total)}
					</span>
				</div>
				<div className="h-3 w-[1px] bg-bfc-border" />
				<div>
					<span className="text-body text-bfc-text2">Agree</span>
					<span className="text-body font-medium text-bfc-text1">
						&nbsp;{bfcDigitsToHumanReadable(proposal.for_votes)}
					</span>
				</div>
				<div className="h-3 w-[1px] bg-bfc-border" />
				<div>
					<span className="text-body text-bfc-text2">Opposition</span>
					<span className="text-body font-medium text-bfc-text1">
						&nbsp;{bfcDigitsToHumanReadable(proposal.against_votes)}
					</span>
				</div>
				<div className="h-3 w-[1px] bg-bfc-border" />
				<div>
					<span className="text-body text-bfc-text2">Quorum</span>
					<span className="text-body font-medium text-bfc-text1">
						&nbsp;{bfcDigitsToHumanReadable(proposal.quorum_votes)}
					</span>
				</div>
			</div>
			<div className="my-3 flex flex-col gap-2">
				<DisclosureBox title="modify proposal" defaultOpen={false}>
					<ModifyProposalObj />
				</DisclosureBox>
				<DisclosureBox title="judge proposal state" defaultOpen={false}>
					<JudgeProposalState />
				</DisclosureBox>
				<DisclosureBox
					title="cast vote"
					defaultOpen={false}
					disabled={proposal.status !== ProposalStatus.Active}
				>
					{votingBfcs.length === 0 ? (
						<Button variant="outline">
							<Link to="/dao">Create Voting BFC</Link>
						</Button>
					) : (
						<CastVote />
					)}
				</DisclosureBox>
				<DisclosureBox
					title="change vote"
					defaultOpen={false}
					disabled={proposal.status !== ProposalStatus.Active || votes.length === 0}
				>
					<ChangeVote />
				</DisclosureBox>
				<DisclosureBox
					title="revoke vote"
					defaultOpen={false}
					disabled={proposal.status !== ProposalStatus.Active || votes.length === 0}
				>
					<RevokeVote />
				</DisclosureBox>
				<DisclosureBox
					title="unvote votes"
					defaultOpen={false}
					disabled={proposal.end_time >= Date.now()}
				>
					<UnvoteVotes />
				</DisclosureBox>
				<DisclosureBox
					title="queue proposal action"
					disabled={!manageKey || proposal.status !== ProposalStatus.Agree}
				>
					<QueueProposalAction />
				</DisclosureBox>
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
	const daoValues = useDaoContext(id || '');
	const { proposal, dao } = daoValues;

	if (!proposal || !dao) {
		return null;
	}

	return (
		<DaoContext.Provider value={daoValues}>
			<div>
				<div className="flex flex-col gap-2 rounded-md border-l-4 border-bfc-border bg-bfc-card p-5 lg:flex-row">
					<div className="flex min-w-0 flex-col gap-1">
						<div className="flex gap-1">
							{proposal.status === ProposalStatus.Agree ? (
								<AgreeSpan />
							) : proposal.status === ProposalStatus.Defeat ? (
								<RejectSpan />
							) : (
								<StatusSpan text={ProposalStatus[proposal.status]} />
							)}
						</div>
						<div className="min-w-0 break-words">
							<Heading as="h2" variant="heading3/semibold" color="bfc-text1" mono>
								{hexToString(dao!.action_record[proposal.action.action_id]?.name || '')}
							</Heading>
						</div>
					</div>
				</div>
				<div className="mt-6 grid grid-cols-2 gap-5">
					<DaoContentDetail />
					<Pool />
				</div>
			</div>
			<Refresh />
		</DaoContext.Provider>
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
