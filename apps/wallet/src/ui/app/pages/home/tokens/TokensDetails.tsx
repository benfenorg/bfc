// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SUI_TYPE_ARG, Coin, type CoinBalance as CoinBalanceType } from '@benfen/bfc.js';
import { useGetCoinBalance, useGetAllBalances } from '@mysten/core';
import { Unpin16, Pin16 } from '@mysten/icons';
import { useMemo } from 'react';
import { Link } from 'react-router-dom';

import { TokenLink } from './TokenLink';
import { TokenList } from './TokenList';
import CoinBalance from './coin-balance';
import { useActiveAddress } from '_app/hooks/useActiveAddress';
=======
import { useIsWalletDefiEnabled } from '_app/hooks/useIsWalletDefiEnabled';
import { LargeButton } from '_app/shared/LargeButton';
>>>>>>> mainnet-v1.24.1
import { Text } from '_app/shared/text';
import { ButtonOrLink } from '_app/shared/utils/ButtonOrLink';
import Alert from '_components/alert';
import { CoinIcon } from '_components/coin-icon';
import Loading from '_components/loading';
import { filterAndSortTokenBalances } from '_helpers';
import {
	useAllowedSwapCoinsList,
	useAppSelector,
	useCoinsReFetchingConfig,
	useSortedCoinsByCategories,
} from '_hooks';
import {
	DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	DELEGATED_STAKES_QUERY_STALE_TIME,
} from '_shared/constants';
import { ampli } from '_src/shared/analytics/ampli';
import { API_ENV } from '_src/shared/api-env';
<<<<<<< HEAD
import { AccountSelector } from '_src/ui/app/components/AccountSelector';
=======
import { FEATURES } from '_src/shared/experimentation/features';
import { AccountsList } from '_src/ui/app/components/accounts/AccountsList';
import { UnlockAccountButton } from '_src/ui/app/components/accounts/UnlockAccountButton';
import { BuyNLargeHomePanel } from '_src/ui/app/components/buynlarge/HomePanel';
import { useActiveAccount } from '_src/ui/app/hooks/useActiveAccount';
>>>>>>> mainnet-v1.24.1
import { usePinnedCoinTypes } from '_src/ui/app/hooks/usePinnedCoinTypes';
import FaucetRequestButton from '_src/ui/app/shared/faucet/FaucetRequestButton';
import PageTitle from '_src/ui/app/shared/PageTitle';
import { useFeature } from '@growthbook/growthbook-react';
import {
	useAppsBackend,
	useBalanceInUSD,
	useCoinMetadata,
	useFormatCoin,
	useGetDelegatedStake,
	useResolveSuiNSName,
} from '@mysten/core';
import { useSuiClientQuery } from '@mysten/dapp-kit';
import { Info12, Pin16, Unpin16 } from '@mysten/icons';
import { type CoinBalance as CoinBalanceType } from '@mysten/sui.js/client';
import { formatAddress, parseStructTag, SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import { useQuery } from '@tanstack/react-query';
import clsx from 'clsx';
import { useEffect, useState, type ReactNode } from 'react';

import Interstitial, { type InterstitialConfig } from '../interstitial';
import { useOnrampProviders } from '../onramp/useOnrampProviders';
import { CoinBalance } from './coin-balance';
import { PortfolioName } from './PortfolioName';
import { TokenIconLink } from './TokenIconLink';
import { TokenLink } from './TokenLink';
import { TokenList } from './TokenList';

type TokenDetailsProps = {
	coinType?: string;
};

function PinButton({ unpin, onClick }: { unpin?: boolean; onClick: () => void }) {
	return (
		<button
			type="button"
			className="border-none bg-transparent text-transparent group-hover/coin:text-steel hover:!text-hero cursor-pointer"
			aria-label={unpin ? 'Unpin Coin' : 'Pin Coin'}
			onClick={(e) => {
				e.preventDefault();
				e.stopPropagation();
				onClick();
			}}
		>
			{unpin ? <Unpin16 /> : <Pin16 />}
		</button>
	);
}

<<<<<<< HEAD
function MyTokens() {
	const accountAddress = useActiveAddress();
=======
function TokenRowButton({
	coinBalance,
	children,
	to,
	onClick,
}: {
	coinBalance: CoinBalanceType;
	children: ReactNode;
	to: string;
	onClick?: () => void;
}) {
	return (
		<ButtonOrLink
			to={to}
			key={coinBalance.coinType}
			onClick={onClick}
			className="no-underline text-subtitle font-medium text-steel hover:font-semibold hover:text-hero"
		>
			{children}
		</ButtonOrLink>
	);
}

export function TokenRow({
	coinBalance,
	renderActions,
	onClick,
}: {
	coinBalance: CoinBalanceType;
	renderActions?: boolean;
	onClick?: () => void;
}) {
	const coinType = coinBalance.coinType;
	const balance = BigInt(coinBalance.totalBalance);
	const [formatted, symbol, { data: coinMeta }] = useFormatCoin(balance, coinType);
	const Tag = onClick ? 'button' : 'div';
	const params = new URLSearchParams({
		type: coinBalance.coinType,
	});
	const allowedSwapCoinsList = useAllowedSwapCoinsList();

	const balanceInUsd = useBalanceInUSD(coinBalance.coinType, coinBalance.totalBalance);

	const isRenderSwapButton = allowedSwapCoinsList.includes(coinType);

	return (
		<Tag
			className={clsx(
				'group flex py-3 pl-1.5 pr-2 rounded hover:bg-sui/10 items-center bg-transparent border-transparent',
				onClick && 'hover:cursor-pointer',
			)}
			onClick={onClick}
		>
			<div className="flex gap-2.5">
				<CoinIcon coinType={coinType} size="md" />
				<div className="flex flex-col gap-1 items-start">
					<Text variant="body" color="gray-90" weight="semibold" truncate>
						{coinMeta?.name || symbol}
					</Text>

					{renderActions && (
						<div className="group-hover:hidden">
							<Text variant="subtitle" color="steel-dark" weight="medium">
								{symbol}
							</Text>
						</div>
					)}

					{renderActions ? (
						<div className="group-hover:flex hidden gap-2.5 items-center">
							<TokenRowButton
								coinBalance={coinBalance}
								to={`/send?${params.toString()}`}
								onClick={() =>
									ampli.selectedCoin({
										coinType: coinBalance.coinType,
										totalBalance: Number(formatted),
									})
								}
							>
								Send
							</TokenRowButton>
							{isRenderSwapButton && (
								<TokenRowButton
									coinBalance={coinBalance}
									to={`/swap?${params.toString()}`}
									onClick={() => {
										ampli.clickedSwapCoin({
											coinType: coinBalance.coinType,
											totalBalance: Number(formatted),
											sourceFlow: 'TokenRow',
										});
									}}
								>
									Swap
								</TokenRowButton>
							)}
						</div>
					) : (
						<div className="flex gap-1 items-center">
							<Text variant="subtitleSmall" weight="semibold" color="gray-90">
								{symbol}
							</Text>
							<Text variant="subtitleSmall" weight="medium" color="steel-dark">
								{formatAddress(coinType)}
							</Text>
						</div>
					)}
				</div>
			</div>

			<div className="ml-auto flex flex-col items-end gap-1">
				{balance > 0n && (
					<Text variant="body" color="gray-90" weight="medium">
						{formatted} {symbol}
					</Text>
				)}

				{balanceInUsd && balanceInUsd > 0 && (
					<Text variant="subtitle" color="steel-dark" weight="medium">
						{Number(balanceInUsd).toLocaleString('en', {
							style: 'currency',
							currency: 'USD',
						})}
					</Text>
				)}
			</div>
		</Tag>
	);
}

export function MyTokens({
	coinBalances,
	isLoading,
	isFetched,
}: {
	coinBalances: CoinBalanceType[];
	isLoading: boolean;
	isFetched: boolean;
}) {
	const isDefiWalletEnabled = useIsWalletDefiEnabled();
>>>>>>> mainnet-v1.24.1
	const apiEnv = useAppSelector(({ app }) => app.apiEnv);
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();
	const { data, isLoading, isFetched } = useGetAllBalances(
		accountAddress,
		staleTime,
		refetchInterval,
		filterAndSortTokenBalances,
	);

	const [_, { pinCoinType, unpinCoinType }] = usePinnedCoinTypes();

<<<<<<< HEAD
	const { recognized, pinned, unrecognized } = useMemo(
		() =>
			data?.reduce(
				(acc, coinBalance) => {
					if (recognizedPackages.includes(coinBalance.coinType.split('::')[0])) {
						acc.recognized.push(coinBalance);
					} else if (pinnedCoinTypes.includes(coinBalance.coinType)) {
						acc.pinned.push(coinBalance);
					} else {
						acc.unrecognized.push(coinBalance);
					}
					return acc;
				},
				{
					recognized: [] as CoinBalanceType[],
					pinned: [] as CoinBalanceType[],
					unrecognized: [] as CoinBalanceType[],
				},
			) ?? { recognized: [], pinned: [], unrecognized: [] },
		[data, recognizedPackages, pinnedCoinTypes],
	);
=======
	const { recognized, pinned, unrecognized } = useSortedCoinsByCategories(coinBalances);
>>>>>>> mainnet-v1.24.1

	const noSuiToken = !data?.find(({ coinType }) => coinType === SUI_TYPE_ARG);

	// Avoid perpetual loading state when fetching and retry keeps failing; add isFetched check.
	const isFirstTimeLoading = isLoading && !isFetched;

	return (
		<Loading loading={isFirstTimeLoading}>
			{recognized.length > 0 && (
				<TokenList title="My Coins" defaultOpen>
					{recognized.map((coinBalance) =>
						isDefiWalletEnabled ? (
							<TokenRow renderActions key={coinBalance.coinType} coinBalance={coinBalance} />
						) : (
							<TokenLink key={coinBalance.coinType} coinBalance={coinBalance} />
						),
					)}
				</TokenList>
			)}

			{pinned.length > 0 && (
				<TokenList title="Pinned Coins" defaultOpen>
					{pinned.map((coinBalance) => (
						<TokenLink
							key={coinBalance.coinType}
							coinBalance={coinBalance}
							centerAction={
								<PinButton
									unpin
									onClick={() => {
										ampli.unpinnedCoin({ coinType: coinBalance.coinType });
										unpinCoinType(coinBalance.coinType);
									}}
								/>
							}
						/>
					))}
				</TokenList>
			)}

			{unrecognized.length > 0 && (
				<TokenList
					title={
						unrecognized.length === 1
							? `${unrecognized.length} Unrecognized Coin`
							: `${unrecognized.length} Unrecognized Coins`
					}
					defaultOpen={apiEnv !== API_ENV.mainnet}
				>
					{unrecognized.map((coinBalance) => (
						<TokenLink
							key={coinBalance.coinType}
							coinBalance={coinBalance}
							subtitle="Send"
							centerAction={
								<PinButton
									onClick={() => {
										ampli.pinnedCoin({ coinType: coinBalance.coinType });
										pinCoinType(coinBalance.coinType);
									}}
								/>
							}
						/>
					))}
				</TokenList>
			)}

			{noSuiToken ? (
				<div className="flex flex-col flex-nowrap justify-center items-center gap-2 text-center mt-7.5">
					<FaucetRequestButton variant="secondary" />
					<Text variant="body" color="bfc-text2" weight="normal">
						To conduct transactions on the BenFen network, you need BFC in your wallet.
					</Text>
				</div>
			) : null}
		</Loading>
	);
}

function getMostNestedName(parsed: ReturnType<typeof parseStructTag>) {
	if (parsed.typeParams.length === 0) {
		return parsed.name;
	}

	if (typeof parsed.typeParams[0] === 'string') {
		return parsed.typeParams[0];
	}

	return getMostNestedName(parsed.typeParams[0]);
}

function getFallbackSymbol(coinType: string) {
	const parsed = parseStructTag(coinType);
	return getMostNestedName(parsed);
}

function TokenDetails({ coinType }: TokenDetailsProps) {
<<<<<<< HEAD
	const activeCoinType = coinType || SUI_TYPE_ARG;
	const accountAddress = useActiveAddress();
=======
	const isDefiWalletEnabled = useIsWalletDefiEnabled();
	const [interstitialDismissed, setInterstitialDismissed] = useState<boolean>(false);
	const activeCoinType = coinType || SUI_TYPE_ARG;
	const activeAccount = useActiveAccount();
	const activeAccountAddress = activeAccount?.address;
	const { data: domainName } = useResolveSuiNSName(activeAccountAddress);
>>>>>>> mainnet-v1.24.1
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();
	const {
		data: coinBalance,
		isError,
		isPending,
		isFetched,
<<<<<<< HEAD
	} = useGetCoinBalance(activeCoinType, accountAddress, refetchInterval, staleTime);

	const tokenBalance = coinBalance?.totalBalance || BigInt(0);
=======
	} = useSuiClientQuery(
		'getBalance',
		{ coinType: activeCoinType, owner: activeAccountAddress! },
		{ enabled: !!activeAccountAddress, refetchInterval, staleTime },
	);

	const { apiEnv } = useAppSelector((state) => state.app);
	const isMainnet = apiEnv === API_ENV.mainnet;
	const { request } = useAppsBackend();
	const { data } = useQuery({
		queryKey: ['apps-backend', 'monitor-network'],
		queryFn: () =>
			request<{ degraded: boolean }>('monitor-network', {
				project: 'WALLET',
			}),
		// Keep cached for 2 minutes:
		staleTime: 2 * 60 * 1000,
		retry: false,
		enabled: isMainnet,
	});

	const {
		data: coinBalances,
		isPending: coinBalancesLoading,
		isFetched: coinBalancesFetched,
	} = useSuiClientQuery(
		'getAllBalances',
		{ owner: activeAccountAddress! },
		{
			enabled: !!activeAccountAddress,
			staleTime,
			refetchInterval,
			select: filterAndSortTokenBalances,
		},
	);

	const { data: delegatedStake } = useGetDelegatedStake({
		address: activeAccountAddress || '',
		staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
		refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	});

	const walletInterstitialConfig = useFeature<InterstitialConfig>(
		FEATURES.WALLET_INTERSTITIAL_CONFIG,
	).value;

	const { providers } = useOnrampProviders();

	const tokenBalance = BigInt(coinBalance?.totalBalance ?? 0);
	const [formatted] = useFormatCoin(tokenBalance, activeCoinType);

	const { data: coinMetadata } = useCoinMetadata(activeCoinType);
	const coinSymbol = coinMetadata ? coinMetadata.symbol : getFallbackSymbol(activeCoinType);
>>>>>>> mainnet-v1.24.1

	// Avoid perpetual loading state when fetching and retry keeps failing add isFetched check
	const isFirstTimeLoading = isPending && !isFetched;

<<<<<<< HEAD
	return (
		<Loading loading={isFirstTimeLoading}>
			{coinType && <PageTitle title={coinSymbol} back="/tokens" />}
			<div
				className="flex flex-col h-full flex-1 flex-grow items-center overflow-y-auto"
				data-testid="coin-page"
			>
				<div className="max-w-full">{!coinType && <AccountSelector />}</div>

				<div data-testid="coin-balance" className="mt-4.5">
					<CoinBalance balance={BigInt(tokenBalance)} type={activeCoinType} />
				</div>
				{isError ? (
					<Alert>
						<div>
							<strong>Error updating balance</strong>
						</div>
					</Alert>
				) : null}
				<div className="flex flex-nowrap gap-2.5 justify-stretch w-full mt-7.5">
					<Link
						className="w-full h-[76px] bg-bfc-card flex flex-col items-center justify-center gap-0.5 rounded-lg border border-solid border-bfc-border no-underline hover:border-bfc"
						data-textid="send-coin-button"
						to={`/send${
							coinBalance?.coinType
								? `?${new URLSearchParams({
										type: coinBalance.coinType,
								  }).toString()}`
								: ''
						}`}
					>
						<svg
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M16.5039 9.414L7.89691 18.021L6.48291 16.607L15.0889 8H7.50391V6H18.5039V17H16.5039V9.414Z"
								fill="#171719"
							/>
						</svg>
						<span className="text-body text-bfc font-medium">Send</span>
					</Link>
					<Link
						className="w-full h-[76px] bg-bfc-card flex flex-col items-center justify-center gap-0.5 rounded-lg border border-solid border-bfc-border no-underline hover:border-bfc"
						to={`/receive`}
					>
						<svg
							width="24"
							height="24"
							viewBox="0 0 24 24"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M9.91007 16.0039L18.5181 7.39691L17.1041 5.98291L8.49707 14.5889V7.00391H6.49707V18.0039H17.4971V16.0039H9.91007Z"
								fill="#09121F"
							/>
						</svg>
						<span className="text-body text-bfc font-medium">Receive</span>
					</Link>
				</div>
				<Link
					to="/stake"
					className="mt-2.5 w-full h-10 bg-bfc-card shrink-0 flex items-center justify-center gap-2.5 rounded-lg border border-solid border-bfc-border no-underline hover:border-bfc"
				>
					<svg
						width="14"
						height="14"
						viewBox="0 0 14 14"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M8.66673 1.16675C9.73421 1.16624 10.7696 1.53172 11.6002 2.20224C12.4308 2.87276 13.0065 3.80778 13.2311 4.85135C13.4558 5.89493 13.3158 6.98399 12.8347 7.93688C12.3535 8.88977 11.5602 9.6489 10.5871 10.0877C10.276 10.7754 9.80278 11.3773 9.20798 11.842C8.61318 12.3066 7.91455 12.6201 7.17201 12.7554C6.42947 12.8908 5.66518 12.844 4.94469 12.6191C4.2242 12.3943 3.569 11.998 3.03529 11.4643C2.50159 10.9306 2.1053 10.2754 1.88041 9.55487C1.65553 8.83438 1.60876 8.07008 1.74411 7.32755C1.87947 6.58501 2.19291 5.88638 2.65756 5.29158C3.12221 4.69678 3.7242 4.22356 4.41189 3.9125C4.78181 3.09402 5.37999 2.39962 6.13468 1.9126C6.88937 1.42558 7.76853 1.16661 8.66673 1.16675ZM6.33339 4.66675C5.87377 4.66675 5.41864 4.75728 4.994 4.93317C4.56936 5.10906 4.18353 5.36687 3.85852 5.69187C3.53351 6.01688 3.27571 6.40272 3.09982 6.82736C2.92392 7.252 2.83339 7.70712 2.83339 8.16675C2.83339 8.62638 2.92392 9.0815 3.09982 9.50614C3.27571 9.93078 3.53351 10.3166 3.85852 10.6416C4.18353 10.9666 4.56936 11.2244 4.994 11.4003C5.41864 11.5762 5.87377 11.6667 6.33339 11.6667C7.26165 11.6667 8.15189 11.298 8.80827 10.6416C9.46465 9.98524 9.83339 9.09501 9.83339 8.16675C9.83339 7.23849 9.46465 6.34825 8.80827 5.69187C8.15189 5.0355 7.26165 4.66675 6.33339 4.66675ZM8.66673 2.33342C8.17246 2.33284 7.6837 2.43721 7.23279 2.63964C6.78188 2.84207 6.37909 3.13794 6.05106 3.50767C6.71038 3.46764 7.37069 3.56804 7.98832 3.80222C8.60594 4.0364 9.16682 4.39902 9.63386 4.86612C10.1009 5.33322 10.4635 5.89414 10.6976 6.51179C10.9317 7.12945 11.032 7.78977 10.9919 8.44908C11.5223 7.97753 11.8968 7.35588 12.0657 6.6666C12.2346 5.97731 12.1901 5.25295 11.9378 4.58959C11.6856 3.92622 11.2377 3.3552 10.6535 2.95226C10.0693 2.54932 9.37642 2.3335 8.66673 2.33342Z"
							fill="#171719"
						/>
					</svg>
					<span className="text-body text-bfc font-medium">Stake</span>
				</Link>

				<MyTokens />
			</div>
		</Loading>
=======
	useEffect(() => {
		const dismissed =
			walletInterstitialConfig?.dismissKey &&
			localStorage.getItem(walletInterstitialConfig.dismissKey);
		setInterstitialDismissed(dismissed === 'true');
	}, [walletInterstitialConfig?.dismissKey]);

	if (
		navigator.userAgent !== 'Playwright' &&
		walletInterstitialConfig?.enabled &&
		!interstitialDismissed
	) {
		return (
			<Interstitial
				{...walletInterstitialConfig}
				onClose={() => {
					setInterstitialDismissed(true);
				}}
			/>
		);
	}
	const accountHasSui = coinBalances?.some(({ coinType }) => coinType === SUI_TYPE_ARG);

	if (!activeAccountAddress) {
		return null;
	}
	return (
		<>
			{isMainnet && data?.degraded && (
				<div className="rounded-2xl bg-warning-light border border-solid border-warning-dark/20 text-warning-dark flex items-center py-2 px-3 mb-4">
					<Info12 className="shrink-0" />
					<div className="ml-2">
						<Text variant="pBodySmall" weight="medium">
							We're sorry that the app is running slower than usual. We're working to fix the issue
							and appreciate your patience.
						</Text>
					</div>
				</div>
			)}

			<Loading loading={isFirstTimeLoading}>
				{coinType && <PageTitle title={coinSymbol} back="/tokens" />}

				<div
					className="flex flex-col h-full flex-1 flex-grow items-center gap-8"
					data-testid="coin-page"
				>
					<AccountsList />
					<BuyNLargeHomePanel />
					<div className="flex flex-col w-full">
						<PortfolioName
							name={activeAccount.nickname ?? domainName ?? formatAddress(activeAccountAddress)}
						/>
						{activeAccount.isLocked ? null : (
							<>
								<div
									data-testid="coin-balance"
									className={clsx(
										'rounded-2xl py-5 px-4 flex flex-col w-full gap-3 items-center mt-4',
										isDefiWalletEnabled ? 'bg-gradients-graph-cards' : 'bg-hero/5',
									)}
								>
									<div className="flex flex-col gap-1 items-center">
										<CoinBalance amount={tokenBalance} type={activeCoinType} />
									</div>

									{!accountHasSui ? (
										<div className="flex flex-col gap-5">
											<div className="flex flex-col flex-nowrap justify-center items-center text-center px-2.5">
												<Text variant="pBodySmall" color="gray-80" weight="normal">
													{isMainnet
														? 'Buy SUI to get started'
														: 'To send transactions on the Sui network, you need SUI in your wallet.'}
												</Text>
											</div>
											<FaucetRequestButton />
										</div>
									) : null}
									{isError ? (
										<Alert>
											<div>
												<strong>Error updating balance</strong>
											</div>
										</Alert>
									) : null}
									<div className="grid grid-cols-3 gap-3 w-full">
										{isMainnet ? (
											<LargeButton
												spacing="sm"
												className={
													!accountHasSui && isMainnet
														? 'col-span-3 !bg-sui-primaryBlue2023 !text-white'
														: ''
												}
												primary={!accountHasSui}
												center
												to="/onramp"
												disabled={(coinType && coinType !== SUI_TYPE_ARG) || !providers?.length}
											>
												Buy
											</LargeButton>
										) : null}

										<LargeButton
											center
											data-testid="send-coin-button"
											to={`/send${
												coinBalance?.coinType
													? `?${new URLSearchParams({
															type: coinBalance.coinType,
													  }).toString()}`
													: ''
											}`}
											disabled={!tokenBalance}
										>
											Send
										</LargeButton>

										<LargeButton
											center
											disabled={!isDefiWalletEnabled || !tokenBalance}
											to={`/swap${
												coinBalance?.coinType
													? `?${new URLSearchParams({
															type: coinBalance.coinType,
													  }).toString()}`
													: ''
											}`}
											onClick={() => {
												if (!coinBalance) {
													return;
												}

												ampli.clickedSwapCoin({
													coinType: coinBalance.coinType,
													totalBalance: Number(formatted),
													sourceFlow: 'LargeButton-TokenDetails',
												});
											}}
										>
											Swap
										</LargeButton>
										{!accountHasSui && (
											<LargeButton disabled to="/stake" center>
												Stake
											</LargeButton>
										)}
									</div>

									<div className="w-full">
										{accountHasSui || delegatedStake?.length ? (
											<TokenIconLink
												disabled={!tokenBalance}
												accountAddress={activeAccountAddress}
											/>
										) : null}
									</div>
								</div>
							</>
						)}
					</div>
					{activeAccount.isLocked ? (
						<UnlockAccountButton account={activeAccount} />
					) : (
						<MyTokens
							coinBalances={coinBalances ?? []}
							isLoading={coinBalancesLoading}
							isFetched={coinBalancesFetched}
						/>
					)}
				</div>
			</Loading>
		</>
>>>>>>> mainnet-v1.24.1
	);
}

export default TokenDetails;
