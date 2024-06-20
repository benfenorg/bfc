// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Button } from '_app/shared/ButtonUI';
import { Text } from '_app/shared/text';
import Overlay from '_components/overlay';
import { useActiveAddress, useCopyToClipboard, useExplorerLink } from '_hooks';
import { ExplorerLinkType } from '_src/ui/app/components/explorer-link/ExplorerLinkType';
import BottomMenuLayout, { Content, Menu } from '_src/ui/app/shared/bottom-menu-layout';
import { ArrowLeft16 } from '@mysten/icons';
import { useNavigate } from 'react-router-dom';

function ReceivePage() {
	const navigate = useNavigate();

	const activeAddress = useActiveAddress();
	const copyCallback = useCopyToClipboard(activeAddress || '', {
		copySuccessMessage: 'Address copied',
	});
	const link = useExplorerLink({ type: ExplorerLinkType.address, useActiveAddress: true });

	return (
		<Overlay showModal title="Receive" closeOverlay={() => navigate('/')}>
			<BottomMenuLayout>
				<Content>
					<div className="flex flex-col items-stretch">
						<div className="flex justify-center">
							<Text variant="body" color="black" weight="normal">
								Receive by scanning the QR code
							</Text>
						</div>
						<div className="my-2.5 mx-auto w-50 h-50 rounded-xl bg-bfc-text1"></div>
						<div
							className="h-10 flex justify-between items-center cursor-pointer"
							onClick={copyCallback}
						>
							<span className="text-[10px]/[18px] font-normal text-bfc-text1 overflow-hidden text-ellipsis whitespace-nowrap">
								{activeAddress}
							</span>
							<div className="shrink-0 w-6 h-6 flex justify-center items-center rounded">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="14"
									height="14"
									viewBox="0 0 14 14"
									fill="none"
								>
									<path
										d="M4.08333 3.50008V1.75008C4.08333 1.59537 4.14479 1.447 4.25419 1.3376C4.36358 1.22821 4.51196 1.16675 4.66667 1.16675H11.6667C11.8214 1.16675 11.9697 1.22821 12.0791 1.3376C12.1885 1.447 12.25 1.59537 12.25 1.75008V9.91675C12.25 10.0715 12.1885 10.2198 12.0791 10.3292C11.9697 10.4386 11.8214 10.5001 11.6667 10.5001H9.91667V12.2501C9.91667 12.5721 9.65417 12.8334 9.32925 12.8334H2.33742C2.26049 12.8339 2.18423 12.8191 2.11302 12.79C2.04181 12.7609 1.97705 12.718 1.92247 12.6638C1.86788 12.6096 1.82455 12.5452 1.79495 12.4742C1.76535 12.4032 1.75008 12.327 1.75 12.2501L1.75175 4.08341C1.75175 3.76141 2.01425 3.50008 2.33917 3.50008H4.08333ZM2.91842 4.66675L2.91667 11.6667H8.75V4.66675H2.91842ZM5.25 3.50008H9.91667V9.33341H11.0833V2.33341H5.25V3.50008Z"
										fill="#09121F"
									/>
								</svg>
							</div>
						</div>
						<a
							className="mt-2.5 h-10 flex justify-between items-center cursor-pointer no-underline"
							href={link!}
							target="_blank"
							rel="noreferrer"
						>
							<span className="text-[10px]/[18px] font-normal text-bfc-text1" onClick={() => {}}>
								View address detail on the official blockchain website.
							</span>
							<div className="w-6 h-6 flex justify-center items-center rounded">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="14"
									height="14"
									viewBox="0 0 14 14"
									fill="none"
								>
									<path
										d="M5.83333 3.5V4.66667H2.91667V11.0833H9.33333V8.16667H10.5V11.6667C10.5 11.8214 10.4385 11.9697 10.3291 12.0791C10.2197 12.1885 10.0714 12.25 9.91667 12.25H2.33333C2.17862 12.25 2.03025 12.1885 1.92085 12.0791C1.81146 11.9697 1.75 11.8214 1.75 11.6667V4.08333C1.75 3.92862 1.81146 3.78025 1.92085 3.67085C2.03025 3.56146 2.17862 3.5 2.33333 3.5H5.83333ZM12.25 1.75V6.41667H11.0833V3.74092L6.53742 8.28742L5.71258 7.46258L10.2579 2.91667H7.58333V1.75H12.25Z"
										fill="#09121F"
									/>
								</svg>
							</div>
						</a>
					</div>
				</Content>
				<Menu stuckClass="receive-coin" className="">
					<Button
						type="button"
						variant="secondary"
						onClick={() => navigate('/')}
						text="Back"
						before={<ArrowLeft16 />}
					/>
				</Menu>
			</BottomMenuLayout>
		</Overlay>
	);
}

export default ReceivePage;
