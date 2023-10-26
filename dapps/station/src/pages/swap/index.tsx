import { useMemo, useState } from 'react';
import { useWalletKit } from '@mysten/wallet-kit';
import { ConnectExcuteButton } from '~/components/ConnectExcuteButton';
import { TokenBox } from '~/components/Swap/TokenBox';
import { TokenPrice } from '~/components/Swap/TokenPrice';
import { SwapInfo } from '~/components/Swap/SwapInfo';
import { ConfirmModal } from '~/components/Swap/ConfirmModal';
import { PriceChart } from '~/components/Swap/PriceChart';

import { ReactComponent as SwapIcon } from '~/assets/swap_icon.svg';
import { ReactComponent as RefreshIcon } from '~/assets/refresh_icon.svg';
import { ReactComponent as FailIcon } from '~/assets/fail_icon.svg';

const SwapDexPage = () => {
	const [amount, setAmount] = useState('');
	const [confirmOpen, setConfirmOpen] = useState(false);
	const { currentAccount } = useWalletKit();
	const [showChart, setShowChart] = useState(false);

	const type = 'mint';
	const data: any = undefined;
	const connectedText = 'Swap';
	const loading = false;

	const demoBfcToken = {
		name: 'BFC',
		type: '0xc8::bfc::BFC',
		logo: 'https://obstatic.243096.com/images/coin/lido-dao.png',
	};

	const demoBusdToken = {
		name: 'BUSD',
		type: '0xc8::usd::USD',
		logo: 'https://obstatic.243096.com/images/coin/lido-dao.png',
	};

	const inputChange = (e: any) => {
		setAmount(e.target.value);
	};

	const swapTip = useMemo(() => {
		return {
			type: 'error',
			message: 'sasassa',
		};
		// {
		// 	type:'warn',
		// 	message:'sasassa'
		// }
	}, [loading]);

	return (
		<div className="flex justify-center">
			<div className="rounded-lg bg-bf-card">
				<div className="flex justify-between items-center py-3 px-2">
					<div className="font-semibold text-bf-text1">兑换</div>
					<RefreshIcon className="cursor-pointer" />
				</div>
				<div className="text-xs bg-white text-bf-text1 rounded-lg w-[400px]">
					<div className="p-5">
						<TokenBox
							showBalance
							title="Send"
							token={demoBfcToken}
							currentAccount={currentAccount}
							maxClick
							amount={amount}
							setAmount={inputChange}
						/>
					</div>
					<div className="relative h-9">
						<div className="w-full absolute top-5 border-b border-bf-border" />
						<div className="w-full absolute left-4">
							<SwapIcon />
						</div>
					</div>
					<div className="p-5 pt-3 border-b border-bf-border">
						<TokenBox
							showBalance
							title="You will receive"
							token={demoBusdToken}
							currentAccount={currentAccount}
							amount={amount}
							readOnly
						/>
					</div>

					<div className="p-5">
						<SwapInfo type="home" />
						<TokenPrice showChart={showChart} setShowChart={setShowChart} fromToken={demoBfcToken} toToken={demoBusdToken} />
						<div className="mt-10">
							<div className="flex items-center text-xs text-bf-red bg-bf-red_10p p-2.5 mb-2.5 rounded-lg">
								<FailIcon className="mr-1" />
								213eqewq
							</div>
							<ConnectExcuteButton
								currentAccount={currentAccount}
								connectedText={connectedText}
								// disabled={!!loading || !Boolean(amount) || !Boolean(data?.result)}
								excute={() => {
									setConfirmOpen(true);
								}}
							/>
						</div>
					</div>

					<ConfirmModal
						isOpen={confirmOpen}
						data={data}
						amount={amount}
						type={type}
						closeModal={() => setConfirmOpen(false)}
						status={'show'}
						fromToken={demoBfcToken}
						toToken={demoBusdToken}
						// errorText={'errorText'}
						confirm={() => {
							// setStatus('waiting');
							// mutate({
							//     address: currentAccount?.address,
							//     type,
							//     amount,
							// });
						}}
					/>
				</div>
			</div>
			<PriceChart  fromToken={demoBfcToken} toToken={demoBusdToken} showChart={showChart} setShowChart={setShowChart}/>
		</div>
	);
};

export default SwapDexPage;
