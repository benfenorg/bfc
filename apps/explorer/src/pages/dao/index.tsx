// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ArrowRight12 } from '@mysten/icons';
import { Heading } from '@mysten/ui';
import { useWalletKit, ConnectButton } from '@mysten/wallet-kit';
import clsx from 'clsx';

import { CreateDaoAction } from './CreateDaoAction';
import { CreateProposal } from './CreateProposal';
import { CreateVotingObc } from './CreateVotingObc';
import { WithdrawVoting } from './WithdrawVoting';
import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useGetDao } from '~/hooks/useGetDao';
import { useGetOBCDaoManageKey } from '~/hooks/useGetOBCDaoManageKey';
import { DisclosureBox } from '~/ui/DisclosureBox';
import { Divider } from '~/ui/Divider';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';

import type { ProposalRecord } from '@mysten/sui.js/client';

function DaoItem({ data }: { data: ProposalRecord }) {
	return (
		<div className="rounded-md border border-obc-border p-5">
			<div className="flex gap-1">
				<AgreeSpan />
				<StatusSpan />
			</div>
			<div className="mt-2 line-clamp-2 h-11 text-ellipsis text-heading4 font-semibold leading-6 text-obc-text1">
				升级starcoin标准库到v11版本，将合约标准库升级到v10升级starcoin标准库到v11版本，将合约标准库升级到v10升级starcoin标准库到v11版本，将合约标准库升级到v10
			</div>
			<div className="mt-2.5">
				<span className="text-body text-obc-text2">ID：</span>
				<span className="text-body text-obc-text1">{data.pid}</span>
			</div>
			<div className="mb-3">
				<span className="text-body text-obc-text2">结束时间：</span>
				<span className="text-body text-obc-text1">{data.end_time}</span>
			</div>
			<Divider type="dashed" />
			<div className="mt-3 flex items-baseline gap-1">
				<div className="text-heading4 font-semibold">{data.for_votes}</div>
				<div className="text-body text-obc-text2">同意</div>
			</div>
			<div className="relative my-3 h-1 overflow-hidden rounded-br-lg rounded-tl-lg bg-obc-green">
				<div className="absolute h-1 w-[50%] bg-obc-red" />
			</div>

			<div className="flex h-4.5 items-center gap-2">
				<div>
					<span className="text-body text-obc-text2">已投票</span>
					<span className="text-body font-medium text-obc-text1">
						{' '}
						{data.for_votes + data.against_votes}
					</span>
				</div>
				<div className="h-3 w-[1px] bg-obc-border" />
				<div>
					<span className="text-body text-obc-text2">法定门槛</span>
					<span className="text-body font-medium text-obc-text1">{data.quorum_votes}</span>
				</div>
			</div>
		</div>
	);
}

function DaoList() {
	const { isConnected, currentAccount } = useWalletKit();
	const { data: manageKey } = useGetOBCDaoManageKey(currentAccount?.address || '');
	console.log(manageKey); // TODO: remove useless code
	const { data, refetch } = useGetDao();

	return (
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
							? '!border !border-solid  !bg-obc !font-mono !text-white'
							: '!flex !flex-nowrap !items-center !gap-1 !bg-obc !font-sans !text-white',
					)}
				/>
			</div>
			{isConnected && (
				<div className="flex flex-col gap-2">
					<DisclosureBox title="create action" defaultOpen={false}>
						<CreateDaoAction refetchDao={refetch} />
					</DisclosureBox>
					{data && (
						<DisclosureBox title="create proposal" defaultOpen={false}>
							<CreateProposal dao={data} refetchDao={refetch} />
						</DisclosureBox>
					)}
					<DisclosureBox title="create voting obc" defaultOpen={false}>
						<CreateVotingObc refetchDao={refetch} />
					</DisclosureBox>
					<DisclosureBox title="withdraw voting obc" defaultOpen={false}>
						<WithdrawVoting refetchDao={refetch} />
					</DisclosureBox>
				</div>
			)}
			<div>
				<Heading variant="heading4/semibold" color="steel-darker">
					提案
				</Heading>
			</div>
			<div className="mt-5 grid grid-cols-3 gap-5">
				{data?.proposal_record.map((item) => (
					<LinkWithQuery key={item.pid} to={`/dao/detail/${item.proposal_uid}`}>
						<DaoItem data={item} />
					</LinkWithQuery>
				))}
			</div>
		</div>
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
						DAO Proposals
					</div>
				),
				size: 'lg',
			}}
			content={
				<div id="home-content">
					<div>
						<ErrorBoundary>
							<div>
								<DaoList />
							</div>
						</ErrorBoundary>
					</div>
				</div>
			}
		/>
	);
}

export default Dao;
