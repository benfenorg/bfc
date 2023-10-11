import { useMemo, useState } from 'react';
import classnames from 'classnames';
import { StationCoinCard } from './StationCoinCard';

function StationMint() {
	return (
		<div className="grid grid-cols-2 gap-5">
			<StationCoinCard type="mint"/>
			<div>asdsd</div>
		</div>
	);
}

function StationWithdraw() {
	return(
		<div className="grid grid-cols-2 gap-5">
			<StationCoinCard type="withdraw"/>
			<div>asdsd</div>
		</div>
	);
}

export function CoinSwap() {
	const [type, setType] = useState<'mint' | 'withdraw'>('mint');

	const token = useMemo(() => {
		if (type === 'mint') return 'OBC';
		return 'OST';
	}, [type]);

	const selectTabCss = 'text-bf-text1 font-semibold bg-white border rounded-md';

	return (
		<div className="mt-5">
			<div className="flex">
				<div
					className={classnames(
						'text-sm px-4 py-1 cursor-pointer',
						type === 'mint' && selectTabCss,
					)}
					onClick={() => setType('mint')}
				>
					铸造
				</div>
				<div
					className={classnames(
						'text-sm px-4 py-1 cursor-pointer',
						type === 'withdraw' && selectTabCss,
					)}
					onClick={() => setType('withdraw')}
				>
					赎回
				</div>
			</div>
			{/* <div className="flex gap-9 items-center">
				{token}
				<div>
					<div>Balance：{balance} {token}</div>
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
			</div> */}
			<div className="mt-5">{type === 'mint' ? <StationMint /> : <StationWithdraw />}</div>
		</div>
	);
}
