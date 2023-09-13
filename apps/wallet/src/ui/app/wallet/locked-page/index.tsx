// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { LockUnlocked16, Logo } from '@mysten/icons';
import { Form, Formik } from 'formik';
import { Link } from 'react-router-dom';
import Browser from 'webextension-polyfill';
import * as Yup from 'yup';

import Alert from '_app/components/alert';
import { Button } from '_app/shared/ButtonUI';
import { CardLayout } from '_app/shared/card-layout';
import FieldLabel from '_app/shared/field-label';
import { Heading } from '_app/shared/heading';
import { PasswordInputField } from '_app/shared/input/password';
import PageMainLayout from '_app/shared/page-main-layout';
import { Text } from '_app/shared/text';
import { unlockWallet } from '_app/wallet/actions';
import { devQuickUnlockEnabled } from '_app/wallet/constants';
import { useLockedGuard } from '_app/wallet/hooks';
import Loading from '_components/loading';
import { useAppDispatch, useInitializedGuard } from '_hooks';
import PageLayout from '_pages/layout';

import st from './LockedPage.module.scss';

let passValidation = Yup.string().ensure();
if (!devQuickUnlockEnabled) {
	passValidation = passValidation.required('Required');
}
const validation = Yup.object({
	password: passValidation,
});

// this is only for dev do not use in prod
async function devLoadPassFromStorage(): Promise<string | null> {
	return (await Browser.storage.local.get({ '**dev**': { pass: null } }))['**dev**']['pass'];
}

export default function LockedPage() {
	const initGuardLoading = useInitializedGuard(true);
	const lockedGuardLoading = useLockedGuard(true);
	const guardsLoading = initGuardLoading || lockedGuardLoading;
	const dispatch = useAppDispatch();
	return (
		<Loading loading={guardsLoading}>
			<PageLayout>
				<PageMainLayout className={st.main}>
					<CardLayout>
						<div className="flex flex-col flex-nowrap items-center justify-center rounded-full w-15 h-15 mb-5">
							<Logo className="bg-obc rounded-full w-15 h-15" />
						</div>
						<Text variant="caption" color="obc-text1" weight="normal">
							Hello There
						</Text>
						<div className="mt-1.25 text-center w-[280px]">
							<Heading variant="heading3" color="black" as="h1" weight="bold" leading="none">
								Welcome Back
							</Heading>
						</div>
						<Formik
							initialValues={{ password: '' }}
							validationSchema={validation}
							validateOnMount={true}
							onSubmit={async ({ password }, { setFieldError }) => {
								if (devQuickUnlockEnabled && password === '') {
									password = (await devLoadPassFromStorage()) || '';
								}
								try {
									await dispatch(unlockWallet({ password })).unwrap();
								} catch (e) {
									setFieldError('password', (e as Error).message || 'Incorrect password');
								}
							}}
						>
							{({ touched, errors, isSubmitting, isValid }) => (
								<Form className={st.form}>
									<FieldLabel txt="Enter Password">
										<PasswordInputField name="password" disabled={isSubmitting} autoFocus />
										{touched.password && errors.password ? <Alert>{errors.password}</Alert> : null}
									</FieldLabel>
									<div className={st.fill} />
									<Button
										type="submit"
										disabled={isSubmitting || !isValid}
										variant="primary"
										size="tall"
										before={<LockUnlocked16 />}
										text="Unlock Wallet"
									/>
									<Link to="/forgot-password" className={st.forgotLink}>
										Forgot password?
									</Link>
								</Form>
							)}
						</Formik>
					</CardLayout>
				</PageMainLayout>
			</PageLayout>
		</Loading>
	);
}
