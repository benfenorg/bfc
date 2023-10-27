import { ModalBase } from '../Modals/Base';
import { Button } from '../Base/Button';
import { SwapInfo } from '~/components/Swap/SwapInfo';
import { Token } from '~/utils/constants';

import { ReactComponent as FailIcon } from '~/assets/fail_icon.svg';
import { ReactComponent as TranslateIcon } from '~/assets/translate_icon.svg';

export type ConfirmModaStatus = 'show' | 'waiting' | 'success' | 'fail';

interface ConfirmModalProps {
	isOpen: boolean;
	type: string;
	data: any;
	amount: any;
	fromToken: Token;
	toToken: Token;
	status: ConfirmModaStatus;
	closeModal: any;
	confirm: any;
	errorText?: string;
}

const showText: any = {
	show: '请确认交易信息',
	waiting: '等待确认中',
	fail: '交易失败',
	success: '交易成功',
};

export function ConfirmModal({
	isOpen,
	fromToken,
	toToken,
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
			title={showText[status]}
			titleLoading={status === 'waiting' ? true : false}
		>
			<div>
				{status === 'show' ? (
					<>
						<div className="p-5 pb-12 border-b border-bf-border">
							<div className="text-xs font-semibold">支付</div>
							<div className="mt-2.5 flex justify-between items-center">
								<div className="text-3xl font-semibold">12,123.00 BFC</div>
								<img className="w-8 h-8 rounded-full" src={fromToken.logo} alt={fromToken.name} />
							</div>
						</div>

						<div className="p-5 pb-12 border-b border-bf-border">
							<div className="text-xs font-semibold">收到</div>
							<div className="mt-2.5 flex justify-between items-center">
								<div className="text-3xl font-semibold">12,123.00 BFC</div>
								<img className="w-8 h-8 rounded-full" src={fromToken.logo} alt={fromToken.name} />
							</div>
						</div>
						<div className="p-5">
							<SwapInfo type="modal" />
						</div>
					</>
				) : (
					<div className="p-5 text-xs">
						<div className="rounded-lg border border-bf-border overflow-hidden font-semibold">
							<div className="py-2.5 bg-bf-card text-center">
								{type === 'mint' ? 'Mint' : 'Withdraw'}
							</div>
							<div className="flex p-2.5 justify-between items-center">
								<img className="w-5 h-5 rounded-full" src={fromToken.logo} alt={fromToken.name} />
								<span>{amount}</span>
							</div>
							<div className="relative h-3 py-0.5">
								<div className="w-full absolute top-1.5 border-b border-bf-border" />
								<div className="w-full absolute left-[50%] translate-x-[-4px]">
									<TranslateIcon />
								</div>
							</div>
							<div className="flex p-2.5 justify-between items-center">
								<img className="w-5 h-5 rounded-full" src={fromToken.logo} alt={fromToken.name} />
								<span>{data?.result}</span>
							</div>
						</div>
					</div>
				)}

				{status !== 'waiting' && (
					<div className="mt-5 p-5">
						{status === 'fail' && errorText && (
							<div className="flex items-center text-xs text-bf-red bg-bf-red_10p p-2.5 mb-2.5 rounded-lg">
								<FailIcon className="mr-1" />
								{errorText}
							</div>
						)}
						{status === 'show' && <Button onClick={confirm}>确认交易</Button>}
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
