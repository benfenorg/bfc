import classnames from 'classnames';
import { Token } from '~/utils/constants';
import { AreaChart, Area, Tooltip, ResponsiveContainer, YAxis, Curve } from 'recharts';
import { chartData } from '../Station/xxx';

import { ReactComponent as ArrowLeftIcon } from '~/assets/arrow_left.svg';
import { ReactComponent as CodeView } from '~/assets/code_view.svg';

interface PriceChart {
	fromToken: Token;
	toToken: Token;
	showChart: boolean;
	setShowChart: (showChart: boolean) => void;
}
export function PriceChart({ fromToken, toToken, showChart, setShowChart }: PriceChart) {
	const chartLaoding = false;
	const TooltipContent = ({ active, payload }: any) => {
		if (active && payload && payload.length) {
			return null;
		}
	};

	const renderChart = () => {
		if (chartLaoding) {
			return (
				<ResponsiveContainer width={'100%'} height={240}>
					<div className="chart-loading">{/* <LoadingIcon /> */}23323</div>
				</ResponsiveContainer>
			);
		}
		if (!chartData || chartData?.length < 1) {
			return (
				<ResponsiveContainer width={'100%'} height={240}>
					<div className="chart-nodata">
						{/* <WarningIcon />
						{t('noDataAvailable')} */}
						sdd
					</div>
				</ResponsiveContainer>
			);
		}
		return (
			<ResponsiveContainer width={'100%'} height={240}>
				<AreaChart data={chartData}>
					<defs>
						<linearGradient id="colorUv" x1="0" y1="0" x2="0" y2="1">
							<stop offset="0%" stopColor="rgba(74, 61, 230)" stopOpacity={0.08} />
							<stop offset="100%" stopColor="rgba(74, 61, 230)" stopOpacity={0} />
						</linearGradient>
					</defs>

					<Tooltip content={TooltipContent} />
					<Curve type="basisOpen" />
					<Area
						type="monotone"
						dataKey="price"
						stroke="#171719"
						strokeWidth={2}
						fillOpacity={1}
						fill="url(#colorUv)"
						//activeDot={<BfcCoinIcon/>}
						activeDot={{ stroke: '#171719', strokeWidth: 3, r: 2 }}
						cursor={'pointer'}
						// dot={{ stroke: "red", strokeWidth: 2 }}
					/>
					<YAxis type="number" hide domain={['dataMin', 'dataMax']} />
				</AreaChart>
			</ResponsiveContainer>
		);
	};
	return (
		<div
			className={classnames(
				'overflow-hidden transition-all',
				showChart ? 'pl-5 w-[680px]' : 'pl-0 w-[0px]',
			)}
		>
			<div className="bg-white rounded-lg text-bf-text1">
				<div className="flex items-center h-[48px] px-[5px] border-b border-bf-border">
					<div className="p-[5px] cursor-pointer" onClick={()=>{setShowChart(false)}}><ArrowLeftIcon /></div>
					<div className="flex items-center cursor-pointer">
						<div className="flex items-center font-semibold text-sm">
							<img className="w-4 h-4 rounded-full" src={fromToken.logo} alt={fromToken.name} />
							<img
								className="w-4 h-4 rounded-full ml-[-2px]"
								src={toToken.logo}
								alt={toToken.name}
							/>
							<span className="ml-1">
								{fromToken.name}/{toToken.name}
							</span>
						</div>
						<CodeView className="ml-1" />
					</div>
				</div>
                <div className="p-5">
				<div className="flex justify-between items-center ">
					<div>
						<div className="text-xl font-bold">8,888.12345</div>
						<div className="font-semibold text-bf-text3 text-sm">2023.6.7 13:22:34</div>
					</div>
					<div className="flex items-center p-0.5 rounded-lg border border-bf-border text-xs text-bf-text2 font-semibold">
						<div className="py-0.5 px-1">24H</div>
						<div className="py-0.5 px-1">7D</div>
						<div className="py-0.5 px-1">30D</div>
						<div className="py-0.5 px-1">1Y</div>
					</div>
				</div>
				<div className="mt-5">{renderChart()}</div>
			</div>
			</div>
		</div>
	);
}
