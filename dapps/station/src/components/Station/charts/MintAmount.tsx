import classNames from 'classnames';
import { AreaChart, Area, Tooltip, ResponsiveContainer, YAxis, Curve } from 'recharts';
import { chartData } from '../xxx';
import { ReactComponent as BfcCoinIcon } from '~/assets/bfc_coin_icon.svg';
import { ReactComponent as BusdCoinIcon } from '~/assets/busd_coin_icon.svg';

export function MintAmount() {
	const chartLaoding = false;
    const TooltipContent = ({ active, payload }: any) => {
        if (active && payload && payload.length) {
        //   const selectValue: any = payload[0];
        //   const selectValueString = selectValue?.value?.toString() || "0";
        //   const useselectValue = new BigNumber(selectValueString).toFixed();
    
        //   const changeString = selectValue?.payload?.timestamp || "";
        //   const usechangeString = dayjs
        //     .unix(changeString)
        //     .format("YYYY/MM/DD HH:mm");
        //   setShowValue(useselectValue || "");
        //   setShowChange(usechangeString || "");
        //   return <div />;
        // }
        // setShowValue("");
        // setShowChange("");
        return null;}
      };
	const renderChart = () => {
		if (chartLaoding) {
			return (
				<ResponsiveContainer width={'100%'} height={160}>
					<div className="chart-loading">{/* <LoadingIcon /> */}23323</div>
				</ResponsiveContainer>
			);
		}
		if (!chartData || chartData?.length < 1) {
			return (
				<ResponsiveContainer width={'100%'} height={160}>
					<div className="chart-nodata">
						{/* <WarningIcon />
						{t('noDataAvailable')} */}
						sdd
					</div>
				</ResponsiveContainer>
			);
		}
		return (
			<ResponsiveContainer width={'100%'} height={160}>
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
		<div className="bg-white text-bf-text1 rounded-lg">
			<div className="flex items-center py-2 px-5 border-b border-bf-border">
				<div className="flex">
					<BusdCoinIcon className="w-[20px] ml-[-2px]" />
				</div>
				<div className="ml-1 font-semibold">13 BUSD</div>
				<div className="ml-1 text-xs text-bf-text2">可铸造数量</div>
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
	);
}
