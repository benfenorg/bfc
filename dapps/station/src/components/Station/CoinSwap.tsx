import { useMemo, useState } from 'react';
import classnames from 'classnames';
import { useStationQuery } from '~/hooks/station';
import { Button } from '../Base/Button';
import { Spinner } from '../Base/Spinner';
import { useSwapMutation } from '../../mutations/station';
import { useWalletKit } from '@mysten/wallet-kit';

function CoinSwap() {
	const [type, setType] = useState<'mint' | 'withdraw'>('mint');
	const [amount, setAmount] = useState('');
	const selectTabCss = 'font-medium';

    const { currentAccount } = useWalletKit();

	const token = useMemo(() => {
		if (type === 'mint') return 'OBC';
		return 'OST';
	}, [type]);

	const { data,isFetching: loading } = useStationQuery(type, amount);

    const swapCoin = useSwapMutation({
        onSuccess: () => {
			// onCreate();
			// toast.success('Kiosk created successfully');
		},
    });

	console.log('estimateestimate', data);
	return (
		<div className="">
			<div className="flex gap-3">
				<div
					className={classnames(
						'border rounded-md px-3 py-1 cursor-pointer',
						type === 'mint' && selectTabCss,
					)}
					onClick={() => setType('mint')}
				>
					铸造
				</div>
				<div
					className={classnames(
						'border rounded-md px-3 py-1 cursor-pointer',
						type === 'withdraw' && selectTabCss,
					)}
					onClick={() => setType('withdraw')}
				>
					赎回
				</div>
			</div>
			<div className="flex gap-9 items-center">
				{token}
				<div>
					<div>Balance：1000 {token}</div>
					<input
						type="text"
						id="search"
						role="search"
						value={amount}
						onChange={(e) => setAmount(e.target.value)}
						className="bg-gray-100 border lg:min-w-[600px] text-gray-900 placeholder:text-gray-500 text-sm rounded rounded-r-none
         focus:ring-transparent 
        focus:border-primary block w-full p-2.5 outline-primary"
						placeholder="Enter Amount"
						required
					/>
				</div>
			</div>
			<div className='flex items-center'>获得：{loading ? <Spinner /> : data}</div>
			<div>
				<Button
					onClick={() => {
                        if(amount && Number(amount) && Number(amount)>0){
                            swapCoin.mutate({
                                address:currentAccount?.address || '',
                                type, 
                                amount
                            })
                        }
                    }}
				>
					执行
				</Button>
			</div>
		</div>
	);
}

export default CoinSwap;
