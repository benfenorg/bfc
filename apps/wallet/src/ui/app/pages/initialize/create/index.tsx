// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Button } from '_app/shared/ButtonUI';
import { CardLayout } from '_app/shared/card-layout';
import { Text } from '_app/shared/text';
import ExternalLink from '_components/external-link';
import { useAppDispatch } from '_hooks';
import PasswordFields from '_pages/initialize/shared/password-fields';
import { createVault } from '_redux/slices/account';
import { ToS_LINK } from '_shared/constants';
import { ampli } from '_src/shared/analytics/ampli';
import { ArrowRight16, Check12 } from '@mysten/icons';
import { Field, Form, Formik } from 'formik';
import { useNavigate } from 'react-router-dom';

import { createMnemonicValidation } from './validation';

const CreatePage = () => {
	const dispatch = useAppDispatch();
	const navigate = useNavigate();
	return (
		<CardLayout title="Create Password for This Wallet" headerCaption="create a new wallet">
			<Formik
				initialValues={{
					terms: false,
					password: '',
					confirmPassword: '',
				}}
				validationSchema={createMnemonicValidation}
				validateOnMount={true}
				onSubmit={async (values) => {
					try {
						await dispatch(createVault({ password: values.password })).unwrap();

						ampli.createdNewWallet();
						navigate('../backup', { state: { onboarding: true } });
					} catch (e) {
						// Do nothing
					}
				}}
			>
				{({ isValid, isSubmitting }) => (
					<Form className="flex flex-col flex-nowrap mt-10 flex-grow w-full">
						<div className="flex flex-col flex-nowrap flex-grow">
							<fieldset disabled={isSubmitting} className="contents">
								<PasswordFields />
								<div className="flex-1" />
								<label className="flex items-center h-5 my-2.5 text-bfc-text1 gap-1.25 relative cursor-pointer">
									<Field name="terms" type="checkbox" id="terms" className="peer/terms invisible" />
									<span className="absolute top-0 left-0.5 h-5 w-5 bg-bfc-card rounded flex justify-center items-center">
										<Check12 />
									</span>
									<span className="absolute top-0 left-0.5 h-5 w-5 bg-bfc-card border border-solid border-bfc-text2 rounded peer-checked/terms:invisible"></span>
									<Text variant="bodySmall" color="bfc-text1" weight="normal">
										I read and agreed to the{' '}
									</Text>
									<ExternalLink href={ToS_LINK} className="text-bfc-text1 font-medium no-underline">
										Terms of Services
									</ExternalLink>
								</label>
							</fieldset>
						</div>
						<Button
							type="submit"
							disabled={!isValid || isSubmitting}
							size="tall"
							text="Create Wallet"
							loading={isSubmitting}
							after={<ArrowRight16 />}
						/>
					</Form>
				)}
			</Formik>
		</CardLayout>
	);
};

export default CreatePage;
