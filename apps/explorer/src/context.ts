// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	type ProposalRecordWithStatus,
	type BfcDao,
	type Vote,
	type VotingBfc,
	ProposalStatus,
} from '@benfen/bfc.js/client';
import { type CoinBalance } from '@benfen/bfc.js/types';
import { useWalletKit } from '@benfen/wallet-kit';
import { useGetCoinBalance } from '@mysten/core';
import * as Sentry from '@sentry/react';
import { createContext, useCallback, useContext, useLayoutEffect, useMemo } from 'react';
// eslint-disable-next-line no-restricted-imports
import { useSearchParams } from 'react-router-dom';

import { useGetBFCDaoManageKey } from './hooks/useGetBFCDaoManageKey';
import { useGetBFCDaoVote } from './hooks/useGetBFCDaoVote';
import { useGetBFCDaoVotingBfc } from './hooks/useGetBFCDaoVotingBfc';
import { useGetDao } from './hooks/useGetDao';
import { Network } from './utils/api/DefaultRpcClient';
import { growthbook } from './utils/growthbook';
import { queryClient } from './utils/queryClient';

export const DEFAULT_NETWORK =
	import.meta.env.VITE_NETWORK || (import.meta.env.DEV ? Network.LOCAL : Network.MAINNET);

export const NetworkContext = createContext<
	[Network | string, (network: Network | string) => void]
>(['', () => null]);

export function useNetworkContext() {
	return useContext(NetworkContext);
}

export function useNetwork(): [string, (network: Network | string) => void] {
	const [searchParams, setSearchParams] = useSearchParams();

	const network = useMemo(() => {
		const networkParam = searchParams.get('network');

		if (networkParam && (Object.values(Network) as string[]).includes(networkParam.toUpperCase())) {
			return networkParam.toUpperCase();
		}

		return networkParam ?? DEFAULT_NETWORK;
	}, [searchParams]);

	const setNetwork = (network: Network | string) => {
		// When resetting the network, we reset the query client at the same time:
		queryClient.cancelQueries();
		queryClient.clear();

		setSearchParams({ network: network.toLowerCase() });
	};

	useLayoutEffect(() => {
		growthbook.setAttributes({
			network,
			environment: import.meta.env.VITE_VERCEL_ENV,
		});

		Sentry.setContext('network', {
			network,
		});
	}, [network]);

	return [network, setNetwork];
}

export interface DaoContextProps {
	manageKey: string;
	dao?: BfcDao;
	proposal?: ProposalRecordWithStatus;
	balance?: CoinBalance;
	votes: Vote[];
	votingBfcs: VotingBfc[];
	refetch: () => void;
}

export const DaoContext = createContext<DaoContextProps | undefined>(undefined);

export const useDaoContext = (proposalId: string): DaoContextProps => {
	const { currentAccount } = useWalletKit();

	const currentAddress = currentAccount?.address || '';

	const { data: daoData, refetch: refetchDao } = useGetDao();
	const { data: votes = [], refetch: refetchVotes } = useGetBFCDaoVote(currentAddress);
	const { data: votingBfcs = [], refetch: refetchVotingBfcs } =
		useGetBFCDaoVotingBfc(currentAddress);
	const { data: balance, refetch: refetchBalance } = useGetCoinBalance(
		'0x2::bfc::BFC',
		currentAddress,
	);
	const { data: manageKey = '' } = useGetBFCDaoManageKey(currentAddress);

	const refetch = useCallback(() => {
		refetchDao();
		refetchVotingBfcs();
		refetchVotes();
		refetchBalance();
	}, [refetchDao, refetchVotingBfcs, refetchVotes, refetchBalance]);

	const proposal = useMemo(() => {
		if (!daoData) {
			return undefined;
		}
		const proposal = daoData.proposal_record.find((i) => i.proposal_uid === proposalId);
		if (!proposal) {
			return undefined;
		}
		return {
			...proposal,
			status: daoData.current_proposal_status[proposal.pid]?.status || ProposalStatus.Pending,
		};
	}, [daoData, proposalId]);

	return {
		manageKey,
		proposal,
		dao: daoData,
		votes,
		votingBfcs,
		refetch,
		balance,
	};
};
