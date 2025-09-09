// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { Button } from '_app/shared/ButtonUI';
import { Text } from '_app/shared/text';
import { AddressInput } from '_components/address-input';
import Overlay from '_src/ui/app/components/overlay';
import { getSignerOperationErrorMessage } from '_src/ui/app/helpers/errorMessages';
import { useChainData } from '_src/ui/app/hooks';
import { useActiveAccount } from '_src/ui/app/hooks/useActiveAccount';
import { useDryRunTransaction } from '_src/ui/app/hooks/useDryRunTransaction';
import { useSigner } from '_src/ui/app/hooks/useSigner';
import BottomMenuLayout, { Content, Menu } from '_src/ui/app/shared/bottom-menu-layout';
import { Transaction } from '@benfen/bfc.js/transactions';
import {
	formatAddress,
	isValidBenfenAddress,
	normalizeStructTag,
	parseStructTag,
} from '@benfen/bfc.js/utils';
import { useGetAllAnonymousCoins } from '@mysten/core';
import { ABFC_TYPE } from '@mysten/core/src/utils/constants';
import { ArrowRight16 } from '@mysten/icons';
import { useMutation } from '@tanstack/react-query';
import clsx from 'clsx';
import { Field, Form, Formik } from 'formik';
import { useMemo } from 'react';
import { toast } from 'react-hot-toast';
import { useNavigate } from 'react-router-dom';
import * as Yup from 'yup';

const initialValues = {
	id: '',
	to: '',
};

type FormValues = typeof initialValues;

const validationSchema = Yup.object({
	id: Yup.string().test('required', 'coin id is a required field', (value) => {
		return !!value;
	}),
	to: Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-sui-address', 'Invalid address. Please check again.', async (value) => {
			return isValidBenfenAddress(value);
		}),
});

export const TransferAstable = () => {
	const navigate = useNavigate();

	const activeAccount = useActiveAccount();
	const dryrun = useDryRunTransaction();
	const signer = useSigner(activeAccount);

	const { ANONYMOUS_STABLE_PKG } = useChainData();
	const { data: anonymousCoins, refetch: refetchCoins } = useGetAllAnonymousCoins(
		activeAccount?.address,
	);

	const coins = useMemo(() => {
		return anonymousCoins?.filter((coin) => {
			const parsed = parseStructTag(coin.balance.type);
			return normalizeStructTag(parsed.typeParams[0]) !== ABFC_TYPE;
		});
	}, [anonymousCoins]);

	const { mutateAsync: transfer } = useMutation({
		mutationKey: ['transfer-anonymous-astables'],
		mutationFn: async (values: FormValues) => {
			const tx = new Transaction();

			tx.moveCall({
				target: `${ANONYMOUS_STABLE_PKG}::anonymous_usd::transfer`,
				typeArguments: [],
				arguments: [tx.object(values.id), tx.pure.address(values.to)],
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
		<Overlay showModal={true} title={'Transfer Anonymous Coins'} closeOverlay={() => navigate('/')}>
			<div className={clsx('flex flex-col w-full h-full')}>
				<div className={clsx('mb-7 flex flex-col gap-2.5')}>
					<Formik
						initialValues={initialValues}
						enableReinitialize={true}
						validateOnMount={true}
						validateOnChange={true}
						validationSchema={validationSchema}
						onSubmit={(values) => transfer(values)}
					>
						{({ submitForm, isSubmitting, isValid, values }) => {
							return (
								<BottomMenuLayout>
									<Content>
										<Form autoComplete={'off'} noValidate={true}>
											<div className="mb-7 flex flex-col gap-2.5">
												<div className="pl-1.5">
													<Text variant="caption" color="steel" weight="semibold">
														Select Coin
													</Text>
												</div>
												<div className="w-full flex relative items-center flex-col">
													<Field as="select" name="id">
														<option value={''} className={'hidden'}></option>
														{coins?.map((i) => (
															<option key={i.id.id} value={i.id.id}>
																{formatAddress(i.id.id)}
															</option>
														))}
													</Field>
												</div>
											</div>
											<div className="w-full flex gap-2.5 flex-col mt-7.5">
												<div className="px-2 tracking-wider">
													<Text variant="caption" color="steel" weight="semibold">
														Enter Recipient Address
													</Text>
												</div>
												<div className="w-full flex relative items-center flex-col">
													<Field component={AddressInput} name="to" placeholder="Enter Address" />
												</div>
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
											text="Transfer"
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
