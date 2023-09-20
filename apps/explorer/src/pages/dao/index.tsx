// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useGetDaoProposals } from '@mysten/core';
import { Heading } from '@mysten/ui';
import { useWalletKit } from '@mysten/wallet-kit';

import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useGetOBCDaoManageKey } from '~/hooks/useGetOBCDaoManageKey';
import { useGetOBCDaoVotingObc } from '~/hooks/useGetOBCDaoVotingObc';
import { useGetOBCDaoVote } from '~/hooks/useGetOBCDaoVote';
import { Divider } from '~/ui/Divider';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';

interface DaoDateItem {
	id: number;
	label: string;
	key: any;
}

interface DaoListProps {
	data: DaoDateItem[];
	isLoading: boolean;
}

function DaoItem({ data }: { data: DaoDateItem }) {
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
				<span className="text-body text-obc-text1">asdsdd</span>
			</div>
			<div className="mb-3">
				<span className="text-body text-obc-text2">结束时间：</span>
				<span className="text-body text-obc-text1">asdsdd</span>
			</div>
			<Divider type="dashed" />
			<div className="mt-3 flex items-baseline gap-1">
				<div className="text-heading4 font-semibold">50.5%</div>
				<div className="text-body text-obc-text2">同意</div>
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

function DaoList({ data, isLoading }: DaoListProps) {
	const { data: list } = useGetDaoProposals();
	console.log('listlist', list);
	return (
		<div>
			<div>
				<Heading variant="heading4/semibold" color="steel-darker">
					提案
				</Heading>
			</div>
			<div className="mt-5 grid grid-cols-3 gap-5">
				{data.map((item) => (
					<LinkWithQuery key={item.id} to="/dao/detail/24324343">
						<DaoItem data={item} />
					</LinkWithQuery>
				))}
			</div>
		</div>
	);
}

function Dao() {
	const data: any = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
	const { currentAccount } = useWalletKit();
	const { data: OBCDaoManageKey } = useGetOBCDaoManageKey(currentAccount?.address || '');
	const { data: OBCDaoVotingObc } = useGetOBCDaoVotingObc(currentAccount?.address || '');
	// vote list
	const { data: OBCDaoVote } = useGetOBCDaoVote(currentAccount?.address || '');
	console.log('OBCDaoVote', OBCDaoVote,OBCDaoVotingObc, OBCDaoManageKey);
	return (
		<PageLayout
			gradientContent={
				<div
					data-testid="home-page"
					className="h-34 flex items-center justify-center text-2xl font-bold text-white xl:h-36"
				>
					DAO Proposals
				</div>
			}
			content={
				<div id="home-content">
					<div>
						<ErrorBoundary>
							<div>
								<DaoList data={data} isLoading={false} />
							</div>
						</ErrorBoundary>
					</div>
				</div>
			}
		/>
	);
}

export default Dao;
