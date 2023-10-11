import { useMemo, useState } from 'react';
import { useWalletKit } from '@mysten/wallet-kit';
import { useStationQuery, useBalance } from '~/hooks/station';
import { ExcuteButton } from '../Base/ExcuteButton';
import { useSwapMutation } from '../../mutations/station';
import { toast } from 'react-hot-toast';

import { ReactComponent as BfcCoinIcon } from '~/assets/bfc_coin_icon.svg';
import { ReactComponent as StationMintIcon } from '~/assets/station_mint_icon.svg';
import { ReactComponent as ViewMoreIcon } from '~/assets/view_more_icon.svg';

export function StationCoinCard({ type }: any) {
	const [amount, setAmount] = useState('');

	const { currentAccount } = useWalletKit();

	const { data: balance } = useBalance(type, currentAccount?.address ?? '');

	const { data, isFetching: loading } = useStationQuery(type, amount);

	const swapCoin = useSwapMutation({
		onSuccess: () => {
			toast.success('Swap 成功');
			// onCreate();
			// toast.success('Kiosk created successfully');
		},
	});


	return (
		<div className="text-xs bg-white text-bf-text1 rounded-md">
			<div className="p-5">
				<div className="flex justify-between">
					<text className="font-semibold">Send</text>
					<div>
						<span className="text-bf-text2">Balance 0.00 BFC</span>
						<span className="ml-2.5 text-bf-link">MAX</span>
					</div>
				</div>
				<div className="mt-3 flex gap-2.5">
					<input
						className="flex-1 text-3xl"
						placeholder="0.00"
						value={amount}
						onChange={(e) => setAmount(e.target.value)}
					/>
					<div className="h-11 flex gap-1 items-center p-1.5 rounded-[31px] border border-bf-border">
						<BfcCoinIcon />
						<span className="text-base font-semibold">BFC</span>
					</div>
				</div>
			</div>
			<div className="relative h-9">
				<div className="w-full absolute top-5 border-b border-bf-border" />
				<div className="w-full absolute left-4">
					{' '}
					<StationMintIcon />
				</div>
			</div>
			<div className="p-5 pt-3">
				<div className="font-semibold">You will receive</div>
				<div className="mt-3 flex gap-2.5">
					<input className="flex-1 text-3xl" placeholder="0.00" />
					<div className="h-11 flex gap-1 items-center p-1.5 rounded-[31px] border border-bf-border">
						<BfcCoinIcon />
						<span className="text-base font-semibold">BUSD</span>
					</div>
				</div>
			</div>
			<div className="relative h-[30px]">
				<div className="w-full absolute top-[15px] border-b border-bf-border" />
				<div className="w-full absolute left-[50%] translate-x-[-15px] cursor-pointer">
					{' '}
					<ViewMoreIcon />
				</div>
			</div>
			<div className="p-5">
				<ExcuteButton currentAccount={currentAccount} />
			</div>
		</div>
	);
}
