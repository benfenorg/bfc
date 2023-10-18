import { ModalBase } from '../Modals/Base';
import { Button } from '../Base/Button';
import { SwapInfo } from './SwapInfo';
import { ReactComponent as BfcCoinIcon } from '~/assets/bfc_coin_icon.svg';
import { ReactComponent as BusdCoinIcon } from '~/assets/busd_coin_icon.svg';
import { ReactComponent as TranslateIcon } from '~/assets/translate_icon.svg';
import { ReactComponent as FailIcon } from '~/assets/fail_icon.svg';

function BfcCoin() {
	return (
		<div className="flex gap-1 items-center p-1.5">
			<BfcCoinIcon className="h-5 w-5" />
			<span className="text-sm font-semibold">BFC</span>
		</div>
	);
}

function BusdCoin() {
	return (
		<div className="flex gap-1 items-center p-1.5">
			<BusdCoinIcon className="h-5 w-5" />
			<span className="text-sm font-semibold">BUSD</span>
		</div>
	);
}

export type ConfirmModaStatus = 'show' | 'waiting' | 'success' | 'fail';

interface ConfirmModalProps {
	isOpen: boolean;
	type: string;
	data: any;
	amount: any;
	status: ConfirmModaStatus;
	closeModal: any;
	confirm: any;
	errorText?: string;
}

const showText: any = {
	mint: {
		button: '确认铸造',
		show: '请确认铸造信息',
		waiting: '等待确认中',
		fail: '铸造失败',
		success: '铸造成功',
	},
	withdraw: {
		button: '确认赎回',
		show: '请确认赎回信息',
		waiting: '等待确认中',
		fail: '赎回失败',
		success: '赎回成功',
	},
};
export function ConfirmModal({
	isOpen,
	type,
	data,
	amount,
	status,
	closeModal,
	confirm,
	errorText,
}: ConfirmModalProps) {
	return (
		<ModalBase
			isOpen={isOpen}
			iconClose={status === 'waiting' ? false : true}
			closeModal={closeModal}
			title={showText[type][status]}
			titleLoading={status === 'waiting' ? true : false}
		>
			<div className="p-5">
				<div className="text-xs">
					<div className="rounded-lg border border-bf-border overflow-hidden font-semibold">
						<div className="py-2.5 bg-bf-card text-center">
							{type === 'mint' ? 'Mint' : 'Withdraw'}
						</div>
						<div className="flex p-2.5 justify-between items-center">
							{type === 'mint' ? <BfcCoin /> : <BusdCoinIcon />}
							<span>{amount}</span>
						</div>
						<div className="relative h-3 py-0.5">
							<div className="w-full absolute top-1.5 border-b border-bf-border" />
							<div className="w-full absolute left-[50%] translate-x-[-4px]">
								<TranslateIcon />
							</div>
						</div>
						<div className="flex p-2.5 justify-between items-center">
							{type === 'mint' ? <BusdCoin /> : <BfcCoin />}
							<span>{data?.result}</span>
						</div>
					</div>
				</div>

				{status === 'show' && (
					<div className="mt-5">
						<SwapInfo type={type} data={data} />
					</div>
				)}

				{status !== 'waiting' && (
					<div className="mt-5">
						{status === 'fail' && errorText && (
							<div className="flex items-center text-xs text-bf-red bg-bf-red_10p p-2.5 mb-2.5 rounded-lg">
								<FailIcon className="mr-1"/>
								{errorText}
							</div>
						)}
						{status === 'show' && <Button onClick={confirm}>确认铸造</Button>}
						{(status === 'fail' || status === 'success') && (
							<Button onClick={closeModal}>知道了</Button>
						)}
						{status === 'success' && (
							<Button onClick={closeModal} className="mt-2.5" ghost>
								前往浏览器上查看
							</Button>
						)}
					</div>
				)}
			</div>
		</ModalBase>
	);
}
