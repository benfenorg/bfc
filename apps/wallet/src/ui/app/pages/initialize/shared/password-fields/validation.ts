// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
	getConfirmPasswordValidation,
	passwordValidation,
} from '_app/shared/input/password/validation';

export const passwordFieldsValidation = {
	password: passwordValidation,
	confirmPassword: getConfirmPasswordValidation('password'),
};
