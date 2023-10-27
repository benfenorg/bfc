import { ReactComponent as BfcCoinIcon } from '~/assets/bfc_coin_icon.svg';

type Token = {
	name: string;
	logo: string;
	type: string;
};
interface TokenBoxProps {
	showBalance?: boolean;
	token?: Token;
	currentAccount?: string;
	amount?: string;
	title?: string;
	maxClick?: boolean;
	readOnly?: boolean;
	setAmount?: (e: any) => void;
}
export function TokenBox({
	showBalance = false,
	token,
	currentAccount,
	amount,
	maxClick,
	readOnly,
	title,
	setAmount,
}: TokenBoxProps) {
	const balance = '11';
	return (
		<>
			<div className="flex justify-between">
				<span className="font-semibold">{title}</span>
				{currentAccount && showBalance ? (
					<div>
						<span className="text-bf-text2">
							Balance {balance} {token?.name}
						</span>
						{maxClick && <span className="ml-2.5 text-bf-link">MAX</span>}
					</div>
				) : null}
			</div>
			<div className="mt-3 flex gap-2.5">
				<div className="flex-1">
					<input
						className="w-full h-full text-3xl"
						placeholder="0.00"
						value={amount}
						readOnly={readOnly}
						onChange={setAmount}
					/>
				</div>
				<div className="h-11 flex gap-1 items-center p-1.5 rounded-[31px] border border-bf-border cursor-pointer">
					<BfcCoinIcon />
					<span className="text-base font-semibold">BUSD</span>
				</div>
			</div>
		</>
	);
}
