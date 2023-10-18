import BigNumber from 'bignumber.js';

export function SwapInfo({type,data}:any) {
	const showcoin = type === 'mint' ? 'BUSD' : 'BFC'
	return (
		<div>
			<div className="bg-bf-card rounded-lg p-2.5 text-xs border border-bf-border">
				<div className="flex justify-between">
					<div className="text-bf-text2">Gas 费</div>
					<div className="font-semibold">{data?.gas} BFC</div>
				</div>
				{data?.profitLoss && new BigNumber(data.profitLoss).abs().isGreaterThan('0.05') ? <div className="mt-2.5 flex justify-between">
					<div className="text-bf-text2">价格影响</div>
					<div className="font-semibold">{data?.profitLoss} %</div>
				</div> : null}
				<div className="mt-2.5 flex justify-between">
					<div className="text-bf-text2">最少获得</div>
					<div className="font-semibold">{data?.result} {showcoin}</div>
				</div>
				<div className="mt-2.5 flex justify-between">
					<div className="text-bf-text2">预期获得</div>
					<div className="font-semibold">{data?.expectAmount} {showcoin}</div>
				</div>
			</div>
		</div>
	);
}
