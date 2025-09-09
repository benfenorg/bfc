// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { Button } from '_app/shared/ButtonUI';
import { Text } from '_app/shared/text';
import Overlay from '_src/ui/app/components/overlay';
import { getSignerOperationErrorMessage } from '_src/ui/app/helpers/errorMessages';
import { useChainData } from '_src/ui/app/hooks';
import { useActiveAccount } from '_src/ui/app/hooks/useActiveAccount';
import { useDryRunTransaction } from '_src/ui/app/hooks/useDryRunTransaction';
import { useSigner } from '_src/ui/app/hooks/useSigner';
import BottomMenuLayout, { Content, Menu } from '_src/ui/app/shared/bottom-menu-layout';
import { InputWithAction } from '_src/ui/app/shared/InputWithAction';
import { Transaction } from '@benfen/bfc.js/transactions';
import { BFC_DECIMALS } from '@benfen/bfc.js/utils';
import { useGetAllAnonymousTreasuryCaps } from '@mysten/core';
import { ArrowRight16 } from '@mysten/icons';
import { useMutation } from '@tanstack/react-query';
import { BigNumber } from 'bignumber.js';
import clsx from 'clsx';
import { Field, Form, Formik } from 'formik';
import { toast } from 'react-hot-toast';
import { useNavigate } from 'react-router-dom';
import * as Yup from 'yup';

const initialValues = {
	capId: '',
	amount: '',
};

type FormValues = typeof initialValues;

const validationSchema = Yup.object({
	capId: Yup.string().required('Cap ID is a required field'),
	amount: Yup.mixed<BigNumber>()
		.transform((_, original) => new BigNumber(original))
		.test('required', `\${path} is a required field`, (value) => {
			return !!value;
		})
		.label('Amount'),
});

export const MintAstable = () => {
	const navigate = useNavigate();

	const activeAccount = useActiveAccount();
	const dryrun = useDryRunTransaction();
	const signer = useSigner(activeAccount);

	const { ANONYMOUS_STABLE_PKG } = useChainData();
	const { data: caps } = useGetAllAnonymousTreasuryCaps(activeAccount?.address);

	const { mutateAsync: mint } = useMutation({
		mutationKey: ['mint-astable'],
		mutationFn: async (values: FormValues) => {
			const tx = new Transaction();
			const bn = new BigNumber(values.amount).shiftedBy(BFC_DECIMALS).toString();
			tx.moveCall({
				target: `${ANONYMOUS_STABLE_PKG}::anonymous_usd::mint`,
				typeArguments: [],
				arguments: [tx.object(values.capId), tx.pure.u64(bn)],
			});

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
						onSubmit={(values) => mint(values)}
					>
						{({ submitForm, isSubmitting, isValid }) => {
							return (
								<BottomMenuLayout>
									<Content>
										<Form autoComplete={'off'} noValidate={true}>
											<div className="w-full flex flex-col flex-grow">
												<div className="px-2 mb-2.5">
													<Text variant="caption" color="steel" weight="semibold">
														Select TreasureCap
													</Text>
												</div>

												<Field as="select" name="capId">
													<option value={''} className={'hidden'}></option>
													{caps?.map((item) => (
														<option key={item.id.id} value={item.id.id} label={item.id.id}></option>
													))}
												</Field>
											</div>
											<div className="w-full flex flex-col flex-grow mt-2.5">
												<div className="px-2 mb-2.5">
													<Text variant="caption" color="steel" weight="semibold">
														Amount
													</Text>
												</div>

												<InputWithAction
													type="numberInput"
													name="amount"
													placeholder="0.00"
													allowNegative={false}
													decimals
													rounded="lg"
													dark
												/>
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
											text="Mint"
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
