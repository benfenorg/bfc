// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import type { AppDispatch } from '_store';
import { useDispatch } from 'react-redux';

export default function useAppDispatch() {
	return useDispatch<AppDispatch>();
}
