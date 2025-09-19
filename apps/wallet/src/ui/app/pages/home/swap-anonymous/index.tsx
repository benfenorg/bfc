// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { Button } from '_app/shared/ButtonUI';
import { Text } from '_app/shared/text';
import Overlay from '_src/ui/app/components/overlay';
import { getSignerOperationErrorMessage } from '_src/ui/app/helpers/errorMessages';
import { useActiveAccount } from '_src/ui/app/hooks/useActiveAccount';
import { useDryRunTransaction } from '_src/ui/app/hooks/useDryRunTransaction';
import { useSigner } from '_src/ui/app/hooks/useSigner';
import BottomMenuLayout, { Content, Menu } from '_src/ui/app/shared/bottom-menu-layout';
import { InputWithAction } from '_src/ui/app/shared/InputWithAction';
import { Transaction } from '@benfen/bfc.js/transactions';
import {
	BFC_DECIMALS,
	BFC_TYPE_ARG,
	normalizeStructTag,
	parseStructTag,
} from '@benfen/bfc.js/utils';
import { useGetAllAnonymousCoins } from '@mysten/core';
import { ArrowRight16 } from '@mysten/icons';
import { useMutation } from '@tanstack/react-query';
import { BigNumber } from 'bignumber.js';
import clsx from 'clsx';
import { Field, Form, Formik } from 'formik';
import { toast } from 'react-hot-toast';
import { useNavigate } from 'react-router-dom';
import * as Yup from 'yup';

const initialValues = {
	pool: '',
	type: '',
	amount: '',
	swapOut: false,
};

type FormValues = typeof initialValues;

const validationSchema = Yup.object({
	pool: Yup.string().required(),
	type: Yup.string().required(),
	amount: Yup.mixed<BigNumber>()
		.transform((_, original) => new BigNumber(original))
		.test('required', `\${path} is a required field`, (value) => {
			return !!value;
		})
		.label('Amount'),
	swapOut: Yup.boolean().required('Swap Out is a required field'),
});

export const SwapAnonymous = () => {
	const navigate = useNavigate();

	const activeAccount = useActiveAccount();
	const dryrun = useDryRunTransaction();
	const signer = useSigner(activeAccount);

	const { data: anonymousCoins, refetch: refetchCoins } = useGetAllAnonymousCoins(
		activeAccount?.address,
	);

	const { mutateAsync: swapInOut } = useMutation({
		mutationKey: ['swap-anonymous-in-out'],
		mutationFn: async (values: FormValues) => {
			const tx = new Transaction();
			const bn = new BigNumber(values.amount).shiftedBy(BFC_DECIMALS).toString();
			if (values.swapOut) {
				const coins = (anonymousCoins || []).filter(
					(i) => normalizeStructTag(parseStructTag(i.balance.type).typeParams[0]) === values.type,
				);
				if (!coins) {
					throw new Error('No coins found');
				}
				const [primary, ...others] = coins;
				if (others.length > 0) {
					tx.moveCall({
						target: `0x2::anonymous_pay::join_vec`,
						typeArguments: [values.type],
						arguments: [
							tx.object(primary.id.id),
							tx.makeMoveVec({ elements: others.map((i) => tx.object(i.id.id)) }),
						],
					});
				}

				tx.moveCall({
					target: `0x2::anonymous_coin::swap_out_with_amount`,
					typeArguments: [values.type, BFC_TYPE_ARG],
					arguments: [tx.object(primary.id.id), tx.pure.u64(bn), tx.object(values.pool)],
				});
			} else {
				tx.moveCall({
					target: `0x2::anonymous_coin::swap_in`,
					typeArguments: [values.type, BFC_TYPE_ARG],
					arguments: [tx.splitCoins(tx.gas, [bn]), tx.object(values.pool)],
				});
			}

			tx.setSenderIfNotSet(activeAccount!.address);
			await dryrun(tx);
			return signer!.signAndExecuteTransactionBlock({
				transactionBlock: tx,
				options: {
					showInput: true,
					showEffects: true,
					showEvents: true,
				},
			});
		},
		onSuccess: (response) => {
			const receiptUrl = `/receipt?txdigest=${encodeURIComponent(
				response!.digest,
			)}&from=transactions`;
			refetchCoins();
			return navigate(receiptUrl);
		},
		onError: (error) => {
			toast.error(
				<div className="max-w-xs overflow-hidden flex flex-col">
					<small className="text-ellipsis overflow-hidden">
						{getSignerOperationErrorMessage(error)}
					</small>
				</div>,
			);
		},
	});

	return (
		<Overlay showModal={true} title={'Swap Anonymous Coins'} closeOverlay={() => navigate('/')}>
			<div className={clsx('flex flex-col w-full h-full')}>
				<div className={clsx('mb-7 flex flex-col gap-2.5')}>
					<Formik
						initialValues={initialValues}
						enableReinitialize={true}
						validateOnMount={true}
						validateOnChange={true}
						validationSchema={validationSchema}
						onSubmit={(values) => swapInOut(values)}
					>
						{({ submitForm, isSubmitting, isValid }) => {
							return (
								<BottomMenuLayout>
									<Content>
										<Form autoComplete={'off'} noValidate={true}>
											<div className="w-full flex flex-col flex-grow">
												<div className="px-2 mb-2.5">
													<Text variant="caption" color="steel" weight="semibold">
														Anonymous Coin Type
													</Text>
												</div>
												<InputWithAction type="text" name="type" placeholder="" rounded="lg" dark />
											</div>

											<div className="w-full flex flex-col flex-grow mt-7.5">
												<div className="px-2 mb-2.5">
													<Text variant="caption" color="steel" weight="semibold">
														Swap Pool
													</Text>
												</div>

												<InputWithAction type="text" name="pool" placeholder="" rounded="lg" dark />
											</div>
											<div className="w-full flex flex-col flex-grow mt-7.5">
												<div className="px-2 mb-2.5">
													<Text variant="caption" color="steel" weight="semibold">
														Select Coin Amount to Swap
													</Text>
												</div>

												<InputWithAction
													type="numberInput"
													name="amount"
													placeholder="0.00"
													suffix={` BFC`}
													allowNegative={false}
													decimals
													rounded="lg"
													dark
												/>
											</div>
											<div className="w-full flex gap-2.5 flex-col mt-7.5">
												<label className={clsx('flex items-center')}>
													<Field type="checkbox" name="swapOut" />
													Swap Out
												</label>
											</div>
										</Form>
									</Content>
									<Menu stuckClass={'sendCoin-cta'} className={'w-full px-0 pb-0 mx-0 gap-2.5'}>
										<Button
											type={'submit'}
											onClick={submitForm}
											variant={'primary'}
											loading={isSubmitting}
											disabled={!isValid || isSubmitting}
											size={'tall'}
											text="Swap"
											after={<ArrowRight16 />}
										/>
									</Menu>
								</BottomMenuLayout>
							);
						}}
					</Formik>
				</div>
			</div>
		</Overlay>
	);
};
