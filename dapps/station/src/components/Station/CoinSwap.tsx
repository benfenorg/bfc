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
			<div className="mt-5">{type === 'mint' ? <StationMint /> : <StationWithdraw />}</div>
		</div>
	);
}
