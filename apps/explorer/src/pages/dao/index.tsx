// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ArrowRight12 } from '@mysten/icons';
import {
	TransactionBlock,
	getExecutionStatusError,
	getExecutionStatusType,
	getTransactionDigest,
	isValidSuiAddress,
} from '@mysten/sui.js';
import { Heading, Button } from '@mysten/ui';
import { useWalletKit, ConnectButton } from '@mysten/wallet-kit';
import { bytesToHex, hexToBytes } from '@noble/hashes/utils';
import { useMutation } from '@tanstack/react-query';
import clsx from 'clsx';
import { z } from 'zod';

import { ErrorBoundary } from '../../components/error-boundary/ErrorBoundary';
import { AgreeSpan, StatusSpan } from '~/components/DaoStatus';
import { PageLayout } from '~/components/Layout/PageLayout';
import { useGetDao } from '~/hooks/useGetDao';
import { useGetOBCDaoManageKey } from '~/hooks/useGetOBCDaoManageKey';
import { useGetOBCDaoVote } from '~/hooks/useGetOBCDaoVote';
import { useGetOBCDaoVotingObc } from '~/hooks/useGetOBCDaoVotingObc';
import { useZodForm } from '~/hooks/useZodForm';
import { DisclosureBox } from '~/ui/DisclosureBox';
import { Divider } from '~/ui/Divider';
import { Input } from '~/ui/Input';
import { Selector } from '~/ui/Selector';
import { LinkWithQuery } from '~/ui/utils/LinkWithQuery';

import type { ProposalRecord } from '@mysten/sui.js/src/client';

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

const createObcdaoSchema = z.object({
	text: z.string().trim().min(1),
});

const createProposalSchema = z.object({
	coin: z.string().trim().min(1),
	action: z.string().trim(),
});

function DaoList() {
	const { isConnected, signAndExecuteTransactionBlock, currentAccount } = useWalletKit();
	const { data: manageKey } = useGetOBCDaoManageKey(currentAccount?.address || '');
	const { data, refetch } = useGetDao();

	const { handleSubmit: createObcdaoHandleSubmit, register: createObcdaoRegister } = useZodForm({
		schema: createObcdaoSchema,
	});

	const createObcdao = useMutation({
		mutationFn: async ({ text }: { text: string }) => {
			const tx = new TransactionBlock();
			tx.moveCall({
				target: `0xc8::obc_system::create_obcdao_action`,
				typeArguments: [],
				arguments: [
					tx.object('OBC00000000000000000000000000000000000000000000000000000000000000c903b9'),
					tx.object(manageKey!),
					tx.object(`0x${bytesToHex(new TextEncoder().encode(text))}`),
				],
			});

			const result = await signAndExecuteTransactionBlock({
				transactionBlock: tx,
			});
			if (getExecutionStatusType(result) === 'failure') {
				throw new Error(getExecutionStatusError(result) || 'Transaction failed');
			}
			return result;
		},
		onSuccess: () => {
			refetch();
		},
	});

	const { handleSubmit: createProposalHandleSubmit, register: createProposalRegister } = useZodForm(
		{
			schema: createProposalSchema,
		},
	);

	const createProposal = useMutation({
		mutationFn: async ({ coin, action }: { coin: string; action: string }) => {
			if (!isValidSuiAddress(coin)) {
				throw new Error('Invalid obc address');
			}
			const tx = new TransactionBlock();
			tx.moveCall({
				target: `0xc8::obc_system::propose`,
				typeArguments: [],
				arguments: [
					tx.object('OBC00000000000000000000000000000000000000000000000000000000000000c903b9'),
					tx.object(manageKey!),
					tx.pure(20),
					tx.object(coin),
					tx.pure(Number.parseInt(action)),
					tx.pure(6000000),
					tx.object('0x6'),
				],
			});

			const result = await signAndExecuteTransactionBlock({
				transactionBlock: tx,
			});
			if (getExecutionStatusType(result) === 'failure') {
				throw new Error(getExecutionStatusError(result) || 'Transaction failed');
			}
			return result;
		},
		onSuccess: () => {
			refetch();
		},
	});

	return (
		<div>
			{manageKey && (
				<div className="flex flex-col gap-2">
					<DisclosureBox title="create obcdao" defaultOpen={false}>
						<form
							onSubmit={createObcdaoHandleSubmit((formData) => {
								createObcdao.mutateAsync(formData).catch((e) => {
									console.error(`failed to create dao`, e);
								});
							})}
							autoComplete="off"
							className="flex flex-col flex-nowrap items-stretch gap-4"
						>
							<Input label="text" {...createObcdaoRegister('text')} />
							<div className="flex items-stretch gap-1.5">
								<Button
									variant="primary"
									type="submit"
									loading={createObcdao.isLoading}
									disabled={!isConnected}
								>
									create
								</Button>
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
							{createObcdao.error ? (
								<div className="">{(createObcdao.error as Error).message || 'Error'}</div>
							) : null}
							{createObcdao.data && <div>{getTransactionDigest(createObcdao.data)}</div>}
						</form>
					</DisclosureBox>
					<DisclosureBox title="create proposal" defaultOpen={false}>
						<form
							onSubmit={createProposalHandleSubmit((formData) => {
								createProposal.mutateAsync(formData).catch((e) => {
									console.error(`failed to create proposal`, e);
								});
							})}
							autoComplete="off"
							className="flex flex-col flex-nowrap items-stretch gap-4"
						>
							<Selector
								{...createProposalRegister('action')}
								label="action"
								options={Object.values(data?.action_record || {}).map((i) => ({
									value: i.action_id.toString(),
									label: new TextDecoder().decode(hexToBytes(i.name.replace(/^0x/, ''))),
								}))}
							/>
							<Input label="coin" {...createProposalRegister('coin')} />
							<div className="flex items-stretch gap-1.5">
								<Button
									variant="primary"
									type="submit"
									loading={createObcdao.isLoading}
									disabled={!isConnected}
								>
									create
								</Button>
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
							{createObcdao.error ? (
								<div className="">{(createObcdao.error as Error).message || 'Error'}</div>
							) : null}
							{createObcdao.data && <div>{getTransactionDigest(createObcdao.data)}</div>}
						</form>
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
					<LinkWithQuery key={item.pid} to={`/dao/detail/${item.pid}`}>
						<DaoItem data={item} />
					</LinkWithQuery>
				))}
			</div>
		</div>
	);
}

function Dao() {
	const { currentAccount } = useWalletKit();
	const { data: OBCDaoVotingObc } = useGetOBCDaoVotingObc(currentAccount?.address || '');
	// vote list
	const { data: OBCDaoVote } = useGetOBCDaoVote(currentAccount?.address || '');
	console.log('OBCDaoVote', OBCDaoVote, OBCDaoVotingObc);
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
