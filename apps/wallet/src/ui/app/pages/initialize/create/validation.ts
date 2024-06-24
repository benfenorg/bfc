// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { passwordFieldsValidation } from '_pages/initialize/shared/password-fields/validation';
import * as Yup from 'yup';

export const createMnemonicValidation = Yup.object({
	...{ terms: Yup.boolean().required().is([true]) },
	...passwordFieldsValidation,
});
