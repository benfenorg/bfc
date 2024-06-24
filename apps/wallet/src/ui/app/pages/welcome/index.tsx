// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BottomMenuLayout, { Content } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import Loading from '_components/loading';
import { useInitializedGuard } from '_hooks';
import PageLayout from '_pages/layout';
import { ampli } from '_src/shared/analytics/ampli';
import { ArrowRight16, BenFenLogoDark, BenFenTextLight } from '@mysten/icons';

const VALUE_PROP = [
	'Send, receive tokens and NFTs',
	'Stake BFC to earn rewards. Help the BenFen network remain decentralized.',
	'Explore apps on BenFen blockchain',
	'Quickly revoke access connection given to apps',
	'Track your BenFen network activity',
];

const WelcomePage = () => {
	const checkingInitialized = useInitializedGuard(false);
	return (
		<PageLayout forceFullscreen={true}>
			<Loading loading={checkingInitialized}>
				<div className="flex flex-col flex-nowrap items-center justify-center">
					<div className="rounded-20 bg-white shadow-wallet-content flex flex-col flex-nowrap items-center justify-center w-popup-width h-popup-height">
						<BottomMenuLayout>
							<Content className="flex flex-col flex-nowrap items-center pt-10 pb-0 px-5">
								<div className="w-[46px] h-[46px] flex justify-center items-center rounded-xl bg-bfc">
									<BenFenLogoDark className="w-[26px]" />
								</div>
								<div className="mt-2.5 flex justify-center items-center">
									<BenFenTextLight className="w-[50px]" />
								</div>

								<div className="mx-auto mt-7.5">
									<div className="text-center">
										<div className="text-xl/[26px] text-bfc-text1 font-bold">
											Welcome to <span className="text-bfc-text2">BenFen</span> Wallet
										</div>
										<div className="mt-2.5 text-xs/[18px] font-normal text-center text-bfc-text2">
											Connecting you to the decentralized web and BenFen network.
										</div>
									</div>

									<div className="mt-7.5 flex gap-1.25 flex-col">
										{VALUE_PROP.map((value) => (
											<div
												key={value}
												className="flex gap-1.25 items-center rounded p-2.5 bg-bfc-card border border-solid border-bfc-border"
											>
												<svg
													className="w-3.5 h-3.5 shrink-0"
													xmlns="http://www.w3.org/2000/svg"
													width="14"
													height="15"
													viewBox="0 0 14 15"
													fill="#171719"
												>
													<path
														d="M7.00002 13.3333C3.77827 13.3333 1.16669 10.7217 1.16669 7.49996C1.16669 4.27821 3.77827 1.66663 7.00002 1.66663C10.2218 1.66663 12.8334 4.27821 12.8334 7.49996C12.8334 10.7217 10.2218 13.3333 7.00002 13.3333ZM7.00002 12.1666C8.2377 12.1666 9.42468 11.675 10.2999 10.7998C11.175 9.92462 11.6667 8.73764 11.6667 7.49996C11.6667 6.26228 11.175 5.0753 10.2999 4.20013C9.42468 3.32496 8.2377 2.83329 7.00002 2.83329C5.76234 2.83329 4.57536 3.32496 3.70019 4.20013C2.82502 5.0753 2.33335 6.26228 2.33335 7.49996C2.33335 8.73764 2.82502 9.92462 3.70019 10.7998C4.57536 11.675 5.76234 12.1666 7.00002 12.1666ZM6.41844 9.83329L3.94335 7.35821L4.76819 6.53338L6.41844 8.18363L9.71777 4.88371L10.5432 5.70854L6.41844 9.83329Z"
														fill="#171719"
													/>
												</svg>
												<div className="text-xs/[18px] font-normal text-bfc-text2">{value}</div>
											</div>
										))}
									</div>
								</div>
							</Content>

							<div className="flex sticky pb-10 m-auto w-[300px] rounded-lg">
								<Button
									to="/initialize/select"
									onClick={() => ampli.clickedGetStarted()}
									size="tall"
									text="Get Started"
									after={<ArrowRight16 />}
								/>
							</div>
						</BottomMenuLayout>
					</div>
				</div>
			</Loading>
		</PageLayout>
	);
};

export default WelcomePage;
