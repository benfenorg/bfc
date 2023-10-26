import { useEffect, useMemo, useState } from 'react';
import { Disclosure } from '@headlessui/react';
import { useWalletKit } from '@mysten/wallet-kit';
import { useStationQuery, useBalance } from '~/hooks/station';
import { useSwapMutation } from '../../mutations/station';
import { toast } from 'react-hot-toast';
import { ConnectExcuteButton } from '../ConnectExcuteButton';
import { ConfirmModal } from './ConfirmModal';
import { SwapInfo } from './SwapInfo';
import { ConfirmModaStatus } from './ConfirmModal';

import { ReactComponent as BfcCoinIcon } from '~/assets/bfc_coin_icon.svg';
import { ReactComponent as BusdCoinIcon } from '~/assets/busd_coin_icon.svg';
import { ReactComponent as StationMintIcon } from '~/assets/station_mint_icon.svg';
import { ReactComponent as StationWithdrawIcon } from '~/assets/station_withdraw_icon.svg';
import { ReactComponent as ViewMoreIcon } from '~/assets/view_more_icon.svg';

function BfcCoin() {
	return (
		<div className="h-11 flex gap-1 items-center p-1.5 rounded-[31px] border border-bf-border">
			<BfcCoinIcon />
			<span className="text-base font-semibold">BFC</span>
		</div>
	);
}

function BusdCoin() {
	return (
		<div className="h-11 flex gap-1 items-center p-1.5 rounded-[31px] border border-bf-border">
			<BusdCoinIcon />
			<span className="text-base font-semibold">BUSD</span>
		</div>
	);
}

export function StationCoinCard({ type }: any) {
	const [amount, setAmount] = useState('');
	const [isOpen, setIsOpen] = useState(false);
	const [status, setStatus] = useState<ConfirmModaStatus>('show');
	const [errorText, setErrorText] = useState<string>('');

	const { currentAccount } = useWalletKit();

	const { data: balance, refetch: balanceRefetch } = useBalance(
		type,
		currentAccount?.address ?? '',
	);

	const { data, isFetching: loading } = useStationQuery(type, amount);

	const { mutate, isLoading } = useSwapMutation({
		onSuccess: () => {
			// toast.success('Swap 成功');
			setStatus('success');
			balanceRefetch();
		},
		onError: (error) => {
			setErrorText(error?.message);
			setStatus('fail');
		},
	});

	useEffect(() => {
		if (!isOpen) {
			setStatus('show');
			setErrorText('');
		}
	}, [isOpen]);

	const connectedText = useMemo(() => {
		const excuteText = type === 'mint' ? '铸造' : '赎回';
		if (loading) {
			return '获取数据...';
		}
		return excuteText;
	}, [loading, type]);

	return (
		<div className="text-xs bg-white text-bf-text1 rounded-lg">
			<div className="p-5">
				<div className="flex justify-between">
					<span className="font-semibold">Send</span>
					{currentAccount ? (
						<div>
							<span className="text-bf-text2">
								Balance {balance} {type === 'mint' ? 'BFC' : 'BUSD'}
							</span>
							<span className="ml-2.5 text-bf-link">MAX</span>
						</div>
					) : null}
				</div>
				<div className="mt-3 flex gap-2.5">
					<input
						className="flex-1 text-3xl"
						placeholder="0.00"
						value={amount}
						onChange={(e) => setAmount(e.target.value)}
					/>
					{type === 'mint' ? <BfcCoin /> : <BusdCoin />}
				</div>
			</div>
			<div className="relative h-9">
				<div className="w-full absolute top-5 border-b border-bf-border" />
				<div className="w-full absolute left-4">
					{type === 'mint' ? <StationMintIcon /> : <StationWithdrawIcon />}
				</div>
			</div>
			<div className="p-5 pt-3">
				<div className="font-semibold">You will receive</div>
				<div className="mt-3 flex gap-2.5">
					<input
						className="flex-1 text-3xl"
						value={data?.result || ''}
						placeholder="0.00"
						readOnly
					/>
					{type === 'mint' ? <BusdCoin /> : <BfcCoin />}
				</div>
			</div>

			<Disclosure defaultOpen>
				{({ open }) => (
					<>
						<Disclosure.Button className="w-full">
							<div className="relative h-[30px]">
								<div className="w-full absolute top-[15px] border-b border-bf-border" />
								<div className="w-full absolute left-[50%] translate-x-[-15px] cursor-pointer">
									<ViewMoreIcon className={`${open ? 'rotate-180 transform' : ''}`} />
								</div>
							</div>
						</Disclosure.Button>
						<Disclosure.Panel className="w-full">
							<div className="mt-[5px] px-5">
								<SwapInfo type={type} data={data} />
							</div>
						</Disclosure.Panel>
					</>
				)}
			</Disclosure>

			<div className="p-5">
				<ConnectExcuteButton
					currentAccount={currentAccount}
					connectedText={connectedText}
					disabled={!!loading || !!isLoading || !Boolean(amount) || !Boolean(data?.result)}
					excute={() => {
						setIsOpen(true);
					}}
				/>
			</div>
			<ConfirmModal
				isOpen={isOpen}
				data={data}
				amount={amount}
				type={type}
				closeModal={() => setIsOpen(false)}
				status={status}
				errorText={errorText}
				confirm={() => {
					setStatus('waiting');
					mutate({
						address: currentAccount?.address,
						type,
						amount,
					});
				}}
			/>
		</div>
	);
}
