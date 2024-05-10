// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { type BfcDao, type ProposalRecord, ProposalStatus } from '@benfen/bfc.js/client';
import { bfcDigitsToHumanReadable, hexToString } from '@benfen/bfc.js/utils';
import { useWalletKit, ConnectButton } from '@benfen/bfc.js';
import { ArrowRight12 } from '@mysten/icons';
import { Heading } from '@mysten/ui';
import clsx from 'clsx';
import dayjs from 'dayjs';

import { CreateDaoAction } from './CreateDaoAction';
import { CreateProposal } from './CreateProposal';
import { CreateVotingBfc } from './CreateVotingBfc';
import { Refresh } from './Refresh';
import { WithdrawVoting } from './WithdrawVoting';
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan, RejectSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { DaoContext, useDaoContext } from '~/context';
import { DisclosureBox } from '~/ui/DisclosureBox';
import { Divider } from '~/ui/Divider';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';

function DaoItem({ data, dao }: { data: ProposalRecord; dao: BfcDao }) {
	const status = dao.current_proposal_status[data.pid]?.status || ProposalStatus.Pending;
	const total = data.for_votes + data.against_votes;

	return (
		<div className="rounded-lg border border-bfc-border p-5 hover:shadow-[0px_24px_24px_0px_rgba(0,0,0,0.04)]">
			<div className="flex gap-1">
				{status === ProposalStatus.Extracted && <AgreeSpan />}
				{status === ProposalStatus.Agree ? (
					<AgreeSpan />
				) : status === ProposalStatus.Defeat ? (
					<RejectSpan />
				) : (
					<StatusSpan text={ProposalStatus[status]} />
				)}
			</div>
			<div className="mt-2 line-clamp-2 h-11 text-ellipsis text-heading4 font-semibold leading-6 text-bfc-text1">
				{hexToString(dao.action_record[data.action.action_id]?.name || '')}
			</div>
			<div className="mt-2.5">
				<span className="text-body text-bfc-text2">ID：</span>
				<span className="text-body text-bfc-text1">{data.pid}</span>
			</div>
			<div className="mb-3">
				<span className="text-body text-bfc-text2">End Time：</span>
				<span className="text-body text-bfc-text1">
					{dayjs(data.end_time).format('YYYY-MM-DD HH:mm:ss')}
				</span>
			</div>
			<Divider type="dashed" />
			<div className="mt-3 flex items-center justify-between">
				<div className="flex items-baseline gap-1">
					<div className="text-heading4 font-semibold">
						{bfcDigitsToHumanReadable(data.for_votes)}
					</div>
					<div className="text-body text-bfc-text2">Agree</div>
				</div>
				<div className="flex items-baseline gap-1">
					<div className="text-heading4 font-semibold">
						{bfcDigitsToHumanReadable(data.against_votes)}
					</div>
					<div className="text-body text-bfc-text2">Opposition</div>
				</div>
			</div>
			<div className="relative my-3 flex h-1 items-stretch overflow-hidden rounded-br-lg rounded-tl-lg bg-bfc-red">
				<div
					className="h-full bg-bfc-green"
					style={{
						width: total === 0 ? '50%' : (data.for_votes / total) * 100 + '%',
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
					<span className="text-body text-bfc-text2">Quorum</span>
					<span className="text-body font-medium text-bfc-text1">
						&nbsp;{bfcDigitsToHumanReadable(data.quorum_votes)}
					</span>
				</div>
			</div>
		</div>
	);
}

function DaoList() {
	const daoValues = useDaoContext('')!;
	const { isConnected } = useWalletKit();

	const { dao, votingBfcs } = daoValues;

	return (
		<DaoContext.Provider value={daoValues}>
			<div className="flex flex-col items-stretch gap-5">
				<div className="self-start">
					<ConnectButton
						connectText={
							<>
								Connect Wallet
								<ArrowRight12 fill="currentColor" className="-rotate-45" />
							</>
						}
						size="md"
						className={clsx(
							'!rounded-md !text-bodySmall',
							isConnected
								? '!border !border-solid  !bg-bfc !font-mono !text-white'
								: '!flex !flex-nowrap !items-center !gap-1 !bg-bfc !font-sans !text-white',
						)}
					/>
				</div>
				{isConnected && (
					<div className="flex flex-col gap-2">
						<DisclosureBox title="create action" defaultOpen={false}>
							<CreateDaoAction />
						</DisclosureBox>
						<DisclosureBox
							title="create proposal"
							defaultOpen={false}
							disabled={Object.values(dao?.action_record || {}).length === 0}
						>
							<CreateProposal />
						</DisclosureBox>
						<DisclosureBox title="create voting bfc" defaultOpen={false}>
							<CreateVotingBfc />
						</DisclosureBox>
						<DisclosureBox
							title="withdraw voting bfc"
							defaultOpen={false}
							disabled={votingBfcs.length === 0}
						>
							<WithdrawVoting />
						</DisclosureBox>
					</div>
				)}
				<div>
					<Heading variant="heading4/semibold" color="steel-darker">
						提案
					</Heading>
				</div>
				<div className="mt-5 grid grid-cols-3 gap-5">
					{dao?.proposal_record.map((item) => (
						<LinkWithQuery key={item.pid} to={`/dao/detail/${item.proposal_uid}`}>
							<DaoItem data={item} dao={dao} />
						</LinkWithQuery>
					))}
				</div>
			</div>
			<Refresh />
		</DaoContext.Provider>
	);
}

function Dao() {
	return (
		<PageLayout
			gradient={{
				content: (
					<div
						data-testid="home-page"
						className="h-34 flex items-center justify-center text-2xl font-bold text-white xl:h-36"
					>
						<span
							style={{
								background: 'linear-gradient(180deg, #FFF 0%, rgba(255, 255, 255, 0.80) 100%)',
								backgroundClip: 'text',
								WebkitBackgroundClip: 'text',
								WebkitTextFillColor: 'transparent',
							}}
						>
							DAO Proposals
						</span>
					</div>
				),
				size: 'lg',
			}}
			content={
				<div id="home-content">
					<div>
						<ErrorBoundary>
							<DaoList />
						</ErrorBoundary>
					</div>
				</div>
			}
		/>
	);
}

export default Dao;
